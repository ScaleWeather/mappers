use crate::ellipsoids::Ellipsoid;
use crate::errors::{
    ensure_finite, ensure_within_range, unpack_required_parameter, ProjectionError,
};
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
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct LambertConformalConic {
    lambda_0: f64,
    n: f64,
    big_f: f64,
    rho_0: f64,
    ellps: Ellipsoid,
}

impl LambertConformalConic {
    /// Initializes builder with default values.
    /// Projection parameters can be set with builder methods,
    /// see documentation of those methods to check which parmeters are required
    /// and default values for optional arguments.
    pub fn builder() -> LambertConformalConicBuilder {
        LambertConformalConicBuilder::default()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct LambertConformalConicBuilder {
    ref_lon: Option<f64>,
    ref_lat: Option<f64>,
    std_parallel_1: Option<f64>,
    std_parallel_2: Option<f64>,
    ellipsoid: Ellipsoid,
}

impl Default for LambertConformalConicBuilder {
    fn default() -> Self {
        Self {
            ref_lon: None,
            ref_lat: None,
            std_parallel_1: None,
            std_parallel_2: None,
            ellipsoid: Ellipsoid::WGS84,
        }
    }
}

impl LambertConformalConicBuilder {
    /// *(required, alternative with [`standard_parallels`](LambertConformalConicBuilder::standard_parallels))* Sets first and second standard parallel (latitude) to the same value. Scale is true along that parallel.
    pub fn single_parallel(&mut self, standard_parallel: f64) -> &mut Self {
        self.std_parallel_1 = Some(standard_parallel);
        self.std_parallel_2 = Some(standard_parallel);
        self
    }

    /// *(required, alternative with [`single_parallel`](LambertConformalConicBuilder::single_parallel))* Sets first and second standard parallel (latitude). Scale is true along those two standard parallels.
    pub fn standard_parallels(&mut self, std_parallel_1: f64, std_parallel_2: f64) -> &mut Self {
        self.std_parallel_1 = Some(std_parallel_1);
        self.std_parallel_2 = Some(std_parallel_2);
        self
    }

    /// *(required)* Sets reference longitude and latitude. Point (0, 0) on the map will be at this coordinates.
    pub fn ref_lonlat(&mut self, lon: f64, lat: f64) -> &mut Self {
        self.ref_lon = Some(lon);
        self.ref_lat = Some(lat);
        self
    }

    /// *(optional)* Sets reference [`Ellipsoid`], defaults to [`WGS84`](Ellipsoid::WGS84).
    pub fn ellipsoid(&mut self, ellps: Ellipsoid) -> &mut Self {
        self.ellipsoid = ellps;
        self
    }

    /// LCC projection constructor.
    ///
    /// To reduce computational overhead of projection functions this
    /// constructor is non-trivial and tries to do as much projection computations as possible.
    /// Thus creating a new structure can involve a significant computational overhead.
    /// When projecting multiple coordinates only one instance of the structure should be created
    /// and copied/borrowed as needed.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError`] with additional information when:
    ///
    /// - one or more longitudes are not within -180..180 range.
    /// - one or more latitudes are not within -90..90 range.
    /// - one or more arguments are not finite.
    /// - absolute value of sum of standard parallels is not positive |`std_par_1` + `std_par_2`| == 0.
    pub fn initialize_projection(&self) -> Result<LambertConformalConic, ProjectionError> {
        let ref_lon = unpack_required_parameter!(self, ref_lon);
        let ref_lat = unpack_required_parameter!(self, ref_lon);
        let std_par_1 = unpack_required_parameter!(self, std_parallel_1);
        let std_par_2 = unpack_required_parameter!(self, std_parallel_2);
        let ellps = self.ellipsoid;
        ensure_finite!(ref_lon, ref_lat, std_par_1, std_par_2);

        ensure_within_range!(ref_lon, -180.0..180.0);
        ensure_within_range!(ref_lat, -90.0..90.0);
        ensure_within_range!(std_par_1, -90.0..90.0);
        ensure_within_range!(std_par_2, -90.0..90.0);

        if approx_eq!(f64, (std_par_1 + std_par_2).abs(), 0.0) {
            return Err(ProjectionError::IncorrectParams(
                "absolute value of sum of standard parallels must be positive",
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

        let n = if approx_eq!(f64, std_par_1, std_par_2) {
            phi_1.sin()
        } else {
            n(m_1, m_2, t_1, t_2)
        };
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
    #[inline]
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

    #[inline]
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
