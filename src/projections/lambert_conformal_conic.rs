use crate::ellipsoids::Ellipsoid;
use crate::errors::ProjectionError;
use crate::Projection;
use float_cmp::approx_eq;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

/// Front-facing struct of Lambert Conformal Conic projection.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct LambertConicConformal {
    lambda_0: f64,
    n: f64,
    big_f: f64,
    rho_0: f64,
    ellps: Ellipsoid,
}

impl LambertConicConformal {
    /// LCC projection constructor from reference longitude, latitude
    /// and two standard parallels.
    pub fn new(
        lon_0: f64,
        lat_0: f64,
        lat_1: f64,
        lat_2: f64,
        ellps: Ellipsoid,
    ) -> Result<Self, ProjectionError> {
        if approx_eq!(f64, lat_1, lat_2) {
            return Err(ProjectionError::IncorrectParams(
                "standard parallels cannot be equal",
            ));
        }

        if !(-180.0..180.0).contains(&lon_0) {
            return Err(ProjectionError::IncorrectParams(
                "longitude must be between -180..180",
            ));
        }

        if !(-90.0..90.0).contains(&lat_1) || !(-90.0..90.0).contains(&lat_2) {
            return Err(ProjectionError::IncorrectParams(
                "latitude must be between -90..90",
            ));
        }

        if !lon_0.is_finite() || !lat_1.is_finite() || !lat_2.is_finite() {
            return Err(ProjectionError::IncorrectParams(
                "one of params is not finite",
            ));
        }

        let phi_0 = lat_0.to_radians();
        let phi_1 = lat_1.to_radians();
        let phi_2 = lat_2.to_radians();

        let t_0 = t(phi_0, ellps);
        let t_1 = t(phi_1, ellps);
        let t_2 = t(phi_2, ellps);
        let m_1 = m(phi_1, ellps);
        let m_2 = m(phi_2, ellps);

        let n = n(m_1, m_2, t_1, t_2);
        let big_f = big_f(m_1, n, t_1);
        let rho_0 = rho(big_f, t_0, n, ellps);

        Ok(LambertConicConformal {
            lambda_0: lon_0.to_radians(),
            n,
            big_f,
            rho_0,
            ellps,
        })
    }
}

impl Projection for LambertConicConformal {
    /// Function to project geographic coordinates
    /// on WGS84 ellipsoid to cartographic coordinates
    /// with previously specified LCC projection.
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

    /// Function to inversly project cartographic coordinates
    /// on specified LCC projection to geographic coordinates
    /// on WGS84 ellipsoid.
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

    fn project(&self, lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError> {
        let (x, y) = self.project_unchecked(lon, lat);

        if !x.is_finite() || !y.is_finite() {
            Err(ProjectionError::InverseProjectionImpossible(lon, lat))
        } else {
            Ok((x, y))
        }
    }

    fn inverse_project(&self, x: f64, y: f64) -> Result<(f64, f64), ProjectionError> {
        let (lon, lat) = self.inverse_project_unchecked(x, y);

        if !lon.is_finite() || !lat.is_finite() {
            Err(ProjectionError::InverseProjectionImpossible(x, y))
        } else {
            Ok((lon, lat))
        }
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

#[cfg(test)]
mod tests {
    use crate::{ellipsoids::WGS84, projections::LambertConicConformal, Projection};

    #[test]
    fn project() {
        let proj = LambertConicConformal::new(18.0, 0.0, 30.0, 60.0, WGS84).unwrap();

        let (lon_0, lat_0) = (18.58973722443749, 54.41412855026378);

        let (x, y) = proj.project(lon_0, lat_0).unwrap();
        let (lon, lat) = proj.inverse_project(x, y).unwrap();

        assert!(lon - lon_0 < 0.000001);
        assert!(lat - lat_0 < 0.000001);
    }
}
