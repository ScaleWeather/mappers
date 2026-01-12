//! An oblique longitude-latitude projection (also known as Rotated Pole) that
//! transforms the Earth's graticule so that the "north pole" of the coordinate system
//! assumes a position different from the true North Pole. This effectively rotates the
//! meridian and parallel grid, allowing any point on Earth to serve as the reference
//! pole of the projection.
//! 
//! The transformation is applied by rotating the spherical coordinate system before
//! applying standard longitude-latitude coordinates. This is particularly useful for
//! regional mapping where the area of interest can be positioned optimally relative
//! to the coordinate grid, reducing distortion in the region of interest.
//!
//! Unlike most other projections in this crate, the output of the
//! [`project`](crate::Projection::project) function is in degrees, not meters.

use crate::Projection;
use crate::errors::ProjectionError;
use crate::errors::{ensure_finite, ensure_within_range, unpack_required_parameter};

#[cfg(feature = "tracing")]
use tracing::instrument;

/// Main projection struct that is constructed from [`ObliqueLonLatBuilder`] and used for computations.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct ObliqueLonLat {
    lambda_p: f64,
    sin_phi_p: f64,
    cos_phi_p: f64,
    lon_0: f64,
}

impl ObliqueLonLat {
    /// Initializes builder with default values.
    /// Projection parameters can be set with builder methods,
    /// refer to the documentation of those methods to check which parameters are required
    /// and default values for optional arguments.
    #[must_use]
    pub fn builder() -> ObliqueLonLatBuilder {
        ObliqueLonLatBuilder::default()
    }
}

/// Builder struct which allows to construct [`ObliqueLonLat`] projection.
/// Refer to the documentation of this struct's methods to check which parameters are required
/// and default values for optional arguments.
#[derive(Debug)]
pub struct ObliqueLonLatBuilder {
    pole_lon: Option<f64>,
    pole_lat: Option<f64>,
    central_lon: f64,
}

impl Default for ObliqueLonLatBuilder {
    fn default() -> Self {
        Self {
            pole_lon: None,
            pole_lat: None,
            central_lon: 0.0,
        }
    }
}

impl ObliqueLonLatBuilder {
    /// *(required)* Sets the longitude and latitude of the rotated pole.
    pub fn pole_lonlat(&mut self, lon: f64, lat: f64) -> &mut Self {
        self.pole_lon = Some(lon);
        self.pole_lat = Some(lat);
        self
    }

    /// *(optional)* Sets the central meridian longitude, defaults to `0.0`.
    pub fn central_lon(&mut self, lon: f64) -> &mut Self {
        self.central_lon = lon;
        self
    }

    /// ObliqueLonLat projection constructor.
    ///
    /// To reduce computational overhead of projection functions this
    /// constructor is non-trivial and tries to do as much projection computations as possible.
    /// Thus creating a new structure can involve a significant computational overhead.
    /// When projecting multiple coordinates only one instance of the structure should be created
    /// and copied/borrowed as needed.
    ///
    /// Ellipsoid is not definable as this projection does not depend on it.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError`] with additional information when:
    ///
    /// - one or more longitudes are not within -180..180 range.
    /// - one or more latitudes are not within -90..90 range.
    /// - one or more arguments are not finite.
    pub fn initialize_projection(&self) -> Result<ObliqueLonLat, ProjectionError> {
        let pole_lon = unpack_required_parameter!(self, pole_lon);
        let pole_lat = unpack_required_parameter!(self, pole_lat);
        let central_lon = self.central_lon;

        ensure_finite!(pole_lon, pole_lat, central_lon);
        ensure_within_range!(pole_lon, -180.0..180.0);
        ensure_within_range!(pole_lat, -90.0..90.0);
        ensure_within_range!(central_lon, -180.0..180.0);

        let phi_p = pole_lat.to_radians();

        Ok(ObliqueLonLat {
            lambda_p: pole_lon.to_radians(),
            sin_phi_p: phi_p.sin(),
            cos_phi_p: phi_p.cos(),
            lon_0: central_lon,
        })
    }
}

fn adjust_lon(lon: f64) -> f64 {
    let pi_degrees = 180.0_f64;
    if lon > pi_degrees {
        lon - 2.0 * pi_degrees
    } else if lon < -pi_degrees {
        lon + 2.0 * pi_degrees
    } else {
        lon
    }
}

impl Projection for ObliqueLonLat {
    #[inline]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64) {
        let lambda = (lon - self.lon_0).to_radians();
        let phi = lat.to_radians();

        let cos_lambda = lambda.cos();
        let sin_lambda = lambda.sin();
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();

        // Formula (5-8b) of [Snyder (1987)](https://pubs.er.usgs.gov/publication/pp1395)
        let lambda_prime = (cos_phi * sin_lambda)
            .atan2(self.sin_phi_p * cos_phi * cos_lambda + self.cos_phi_p * sin_phi) + self.lambda_p;
        
        // Formula (5-7)
        let phi_prime = (self.sin_phi_p * sin_phi - self.cos_phi_p * cos_phi * cos_lambda).asin();

        let lon_prime = adjust_lon(lambda_prime.to_degrees());
        let lat_prime = phi_prime.to_degrees();
        return (lon_prime, lat_prime);
    }

    #[inline]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        let lambda_prime = x.to_radians() - self.lambda_p;
        let phi_prime = y.to_radians();

        let cos_lambda_prime = lambda_prime.cos();
        let sin_lambda_prime = lambda_prime.sin();
        let cos_phi_prime = phi_prime.cos();
        let sin_phi_prime = phi_prime.sin();

        // Formula (5-10b)
        let lambda = (cos_phi_prime * sin_lambda_prime)
            .atan2(self.sin_phi_p * cos_phi_prime * cos_lambda_prime - self.cos_phi_p * sin_phi_prime);

        // Formula (5-9)
        let phi = (self.sin_phi_p * sin_phi_prime + self.cos_phi_p * cos_phi_prime * cos_lambda_prime).asin();

        let lon  = adjust_lon(lambda.to_degrees() + self.lon_0);
        let lat = phi.to_degrees();
        return (lon, lat);
    }
}
