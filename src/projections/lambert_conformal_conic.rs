use crate::ellipsoids::Ellipsoid;
use crate::errors::ProjectionError;
use crate::Projection;
use float_cmp::approx_eq;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

/// Lambert Conformal Conic projection (LCC) is a conic map projection used for aeronautical charts,
/// portions of the State Plane Coordinate System, and many national and regional
/// mapping systems [(Wikipedia, 2022)](https://en.wikipedia.org/wiki/Lambert_conformal_conic_projection).
///
/// Summary by [Snyder (1987)](https://pubs.er.usgs.gov/publication/pp1395):
///
/// - Conic.
/// - Conformal.
/// - Parallels are unequally spaced arcs of concentric circles, more closely spaced near the center of the map.
/// - Meridians are equally spaced radii of the same circles, thereby cutting parallels at right angles.
/// - Scale is true along two standard parallels, normally, or along just one.
/// - Pole in same hemisphere as standard parallels is a point; other pole is at infinity.
/// - Used for maps of countries and regions with predominant east-west expanse.
/// - Presented by Lambert in 1772.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct LambertConformalConic {
    lambda_0: f64,
    n: f64,
    big_f: f64,
    rho_0: f64,
    ellps: Ellipsoid,
}

impl LambertConformalConic {
    /// LCC projection constructor. 
    /// 
    /// To reduce computational overhead of projection functions this 
    /// constructor is non-trivial and tries to do as much projection computations as possible.
    /// Thus creating a new structure can involve a significant computational overhead.
    /// When projecting multiple coordinates only one instance of the structure should be created
    /// and cloned/borrowed as needed.
    ///
    /// # Arguments
    ///
    /// - `ref_lon`, `ref_lat` - Reference longitude and latitude. Point (0, 0) on the map will be at this coordinates.
    /// - `std_par_1`, `std_par_1` - First and second standard parallel (latitude). Scale is true along two standard parallels.
    /// - `ellps` - Reference [`Ellipsoid`].
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError::IncorrectParams`] with additional information when:
    ///
    /// - standard parallels are equal.
    /// - one or more longitudes are not within -180..180 range.
    /// - one or more latitudes are not within -90..90 range.
    /// - one or more arguments are not finite.
    pub fn new(
        ref_lon: f64,
        ref_lat: f64,
        std_par_1: f64,
        std_par_2: f64,
        ellps: Ellipsoid,
    ) -> Result<Self, ProjectionError> {
        if approx_eq!(f64, std_par_1, std_par_2) {
            return Err(ProjectionError::IncorrectParams(
                "standard parallels cannot be equal",
            ));
        }

        if !(-180.0..180.0).contains(&ref_lon) {
            return Err(ProjectionError::IncorrectParams(
                "longitude must be between -180..180",
            ));
        }

        if !(-90.0..90.0).contains(&ref_lat)
            || !(-90.0..90.0).contains(&std_par_1)
            || !(-90.0..90.0).contains(&std_par_2)
        {
            return Err(ProjectionError::IncorrectParams(
                "latitude must be between -90..90",
            ));
        }

        if !ref_lon.is_finite()
            || !ref_lat.is_finite()
            || !std_par_1.is_finite()
            || !std_par_2.is_finite()
        {
            return Err(ProjectionError::IncorrectParams(
                "one of arguments is not finite",
            ));
        }

        let phi_0 = ref_lat.to_radians();
        let phi_1 = std_par_1.to_radians();
        let phi_2 = std_par_2.to_radians();

        let t_0 = t(phi_0, ellps);
        let t_1 = t(phi_1, ellps);
        let t_2 = t(phi_2, ellps);
        let m_1 = m(phi_1, ellps);
        let m_2 = m(phi_2, ellps);

        let n = n(m_1, m_2, t_1, t_2);
        let big_f = big_f(m_1, n, t_1);
        let rho_0 = rho(big_f, t_0, n, ellps);

        Ok(LambertConformalConic {
            lambda_0: ref_lon.to_radians(),
            n,
            big_f,
            rho_0,
            ellps,
        })
    }
}

impl Projection for LambertConformalConic {
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64) {
        let phi = lat.to_radians();
        let lambda = lon.to_radians();

        let t = t(phi, self.ellps);
        let theta = self.n * (lambda - self.lambda_0);
        let rho = rho(self.big_f, t, self.n, self.ellps);

        let x = rho * theta.sin();
        let y = self.rho_0 - rho * theta.cos();

        (x, y)
    }

    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        let rho = (self.n.signum()) * (x.powi(2) + (self.rho_0 - y).powi(2)).sqrt();

        let theta;
        {
            // adjusting signs locally for theta
            let sign = self.n.signum();
            let x = x * sign;
            let y = y * sign;
            let rho_0 = self.rho_0 * sign;
            theta = (x / (rho_0 - y)).atan();
        }

        let t = (rho / (self.ellps.A * self.big_f)).powf(1.0 / self.n);

        let lambda = (theta / self.n) + self.lambda_0;
        let phi = phi_for_inverse(t, self.ellps);

        (lambda.to_degrees(), phi.to_degrees())
    }
}

fn t(phi: f64, ellps: Ellipsoid) -> f64 {
    ((FRAC_PI_4 - 0.5 * phi).tan())
        / (((1.0 - ellps.E * phi.sin()) / (1.0 + ellps.E * phi.sin())).powf(ellps.E / 2.0))
}

fn m(phi: f64, ellps: Ellipsoid) -> f64 {
    phi.cos() / (1.0 - (ellps.E.powi(2) * (phi.sin()).powi(2))).sqrt()
}

fn n(m_1: f64, m_2: f64, t_1: f64, t_2: f64) -> f64 {
    (m_1.ln() - m_2.ln()) / (t_1.ln() - t_2.ln())
}

fn big_f(m_1: f64, n: f64, t_1: f64) -> f64 {
    m_1 / (n * t_1.powf(n))
}

fn rho(big_f: f64, t: f64, n: f64, ellps: Ellipsoid) -> f64 {
    ellps.A * big_f * t.powf(n)
}

/// To compute the phi for inverse projection
/// truncated infinite series is used with
/// optimisations for reducing trigonometric
/// functions calls.
fn phi_for_inverse(t: f64, ellps: Ellipsoid) -> f64 {
    let chi = FRAC_PI_2 - 2.0 * t.atan();

    let big_a = (ellps.E.powi(2) / 2.0)
        + 5.0 * (ellps.E.powi(4) / 24.0)
        + (ellps.E.powi(6) / 12.0)
        + 13.0 * (ellps.E.powi(8) / 360.0);

    let big_b = 7.0 * (ellps.E.powi(4) / 48.0)
        + 29.0 * (ellps.E.powi(6) / 240.0)
        + 811.0 * (ellps.E.powi(8) / 11520.0);

    let big_c = 7.0 * (ellps.E.powi(6) / 120.0) + 81.0 * (ellps.E.powi(8) / 1120.0);

    let big_d = 4279.0 * (ellps.E.powi(8) / 161_280.0);

    let a_prime = big_a - big_c;
    let b_prime = 2.0 * big_b - 4.0 * big_d;
    let c_prime = 4.0 * big_c;
    let d_prime = 8.0 * big_d;

    let sin_2chi = (2.0 * chi).sin();
    let cos_2chi = (2.0 * chi).cos();

    chi + (sin_2chi
        * (a_prime + (cos_2chi * (b_prime + (cos_2chi * (c_prime + (d_prime * cos_2chi)))))))
}
