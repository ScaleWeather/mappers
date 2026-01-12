//! This is a modified version of the [`Azimuthal Equidistant`](crate::projections::azimuthal_equidistant) projection,
//! defined for islands of Micronesia and described by [Snyder (1987)](https://pubs.er.usgs.gov/publication/pp1395).
//! It does not use Geodesic calculations so it is faster than the original projection, but significantly
//! diverges from the AEQD projection at bigger scales.
//!
//! The azimuthal equidistant projection is an azimuthal map projection.
//! It has the useful properties that all points on the map are at proportionally
//! correct distances from the center point, and that all points on the map are at the
//! correct azimuth (direction) from the center point. A useful application for this
//! type of projection is a polar projection which shows all meridians (lines of longitude) as straight,
//! with distances from the pole represented correctly [(Wikipedia, 2022)](https://en.wikipedia.org/wiki/Azimuthal_equidistant_projection).
//!
//! Summary by [Snyder (1987)](https://pubs.er.usgs.gov/publication/pp1395):
//!
//! - Azimuthal.
//! - Distances measured from the center are true.
//! - Distances not measured along radii from the center are not correct.
//! - The center of projection is the only point without distortion.
//! - Directions from the center are true (except on some oblique and equatorial ellipsoidal forms).
//! - Neither equal-area nor conformal.
//! - All meridians on the polar aspect, the central meridian on other aspects, and the Equator on the equatorial aspect are straight lines.
//! - Parallels on the polar projection are circles spaced at true intervals (equidistant for the sphere).
//! - The outer meridian of a hemisphere on the equatorial aspect (for the sphere) is a circle.
//! - All other meridians and parallels are complex curves.
//! - Not a perspective projection.
//! - Point opposite the center is shown as a circle (for the sphere) surrounding the map.
//! - Used in the polar aspect for world maps and maps of polar hemispheres.
//! - Used in the oblique aspect for atlas maps of continents and world maps for aviation and radio use.
//! - Known for many centuries in the polar aspect.

use float_cmp::approx_eq;

use crate::{
    Projection, ProjectionError,
    ellipsoids::Ellipsoid,
    errors::{ensure_finite, ensure_within_range, unpack_required_parameter},
};

#[cfg(feature = "tracing")]
use tracing::instrument;

/// Main projection struct that is constructed from [`ModifiedAzimuthalEquidistantBuilder`] and used for computations.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct ModifiedAzimuthalEquidistant {
    lon_0: f64,
    lat_0: f64,
    n_1: f64,
    g: f64,
    ellps: Ellipsoid,
}

impl ModifiedAzimuthalEquidistant {
    /// Initializes builder with default values.
    /// Projection parameters can be set with builder methods,
    /// refer to the documentation of those methods to check which parmeters are required
    /// and default values for optional arguments.
    #[must_use]
    pub fn builder() -> ModifiedAzimuthalEquidistantBuilder {
        ModifiedAzimuthalEquidistantBuilder::default()
    }
}

/// Builder struct which allows to construct [`ModifiedAzimuthalEquidistant`] projection.
/// Refer to the documentation of this struct's methods to check which parmeters are required
/// and default values for optional arguments.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct ModifiedAzimuthalEquidistantBuilder {
    ref_lon: Option<f64>,
    ref_lat: Option<f64>,
    ellipsoid: Ellipsoid,
}

impl Default for ModifiedAzimuthalEquidistantBuilder {
    fn default() -> Self {
        Self {
            ref_lon: None,
            ref_lat: None,
            ellipsoid: Ellipsoid::WGS84,
        }
    }
}

impl ModifiedAzimuthalEquidistantBuilder {
    /// *(required)* Sets reference longitude and latitude. Point (0, 0) on the map will be at this coordinates.
    pub const fn ref_lonlat(&mut self, lon: f64, lat: f64) -> &mut Self {
        self.ref_lon = Some(lon);
        self.ref_lat = Some(lat);
        self
    }

    /// *(optional)* Sets reference [`Ellipsoid`], defaults to [`WGS84`](Ellipsoid::WGS84).
    pub const fn ellipsoid(&mut self, ellps: Ellipsoid) -> &mut Self {
        self.ellipsoid = ellps;
        self
    }

    /// `ModifiedAzimuthalEquidistant` projection constructor.
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
    pub fn initialize_projection(&self) -> Result<ModifiedAzimuthalEquidistant, ProjectionError> {
        let ref_lon = unpack_required_parameter!(self, ref_lon);
        let ref_lat = unpack_required_parameter!(self, ref_lat);
        let ellps = self.ellipsoid;
        ensure_finite!(ref_lon, ref_lat);

        ensure_within_range!(ref_lon, -180.0..180.0);
        ensure_within_range!(ref_lat, -90.0..90.0);

        let lon_0 = ref_lon.to_radians();
        let lat_0 = ref_lat.to_radians();

        let n_1 = ellps.A / ellps.E.powi(2).mul_add(-(lat_0.sin()).powi(2), 1.0).sqrt();
        let g = ellps.E * lat_0.sin() / ellps.E.mul_add(-ellps.E, 1.0).sqrt();

        Ok(ModifiedAzimuthalEquidistant {
            lon_0,
            lat_0,
            n_1,
            g,
            ellps,
        })
    }
}

impl Projection for ModifiedAzimuthalEquidistant {
    #[inline]
    #[allow(clippy::many_single_char_names)]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64) {
        let lon = lon.to_radians();
        let lat = lat.to_radians();

        let n = self.ellps.A / self.ellps.E.powi(2).mul_add(-(lat.sin()).powi(2), 1.0).sqrt();

        let psi = self.ellps.E.mul_add(-self.ellps.E, 1.0).mul_add(lat.tan(), (self.ellps.E.powi(2) * self.n_1 * self.lat_0.sin()) / (n * lat.cos()))
        .atan();

        let az = ((lon - self.lon_0).sin())
            .atan2(self.lat_0.cos().mul_add(psi.tan(), -(self.lat_0.sin() * (lon - self.lon_0).cos())));

        let s = if approx_eq!(f64, az.sin(), 0.0) {
            self.lat_0.cos().mul_add(psi.sin(), -(self.lat_0.sin() * psi.cos()))
                .asin()
                .abs()
                * az.cos().signum()
        } else {
            (((lon - self.lon_0).sin() * psi.cos()) / (az.sin())).asin()
        };

        let h = self.ellps.E * self.lat_0.cos() * az.cos() / self.ellps.E.mul_add(-self.ellps.E, 1.0).sqrt();

        let c = (self.n_1 * s)
            * ((s.powi(5) / 48.0) * self.g).mul_add(-h, (s.powi(4) / 120.0).mul_add(h.powi(2).mul_add(7.0f64.mul_add(-h.powi(2), 4.0), -(3.0 * self.g.powi(2) * 7.0f64.mul_add(-h.powi(2), 1.0))), ((s.powi(3) / 8.0) * self.g * h).mul_add(2.0f64.mul_add(-h.powi(2), 1.0), 1.0 - (s.powi(2) * h.powi(2) * h.mul_add(-h, 1.0) / 6.0))));

        let x = c * az.sin();
        let y = c * az.cos();

        (x, y)
    }

    #[inline]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        let c = x.hypot(y);
        let az = x.atan2(y);

        let big_a =
            -self.ellps.E * self.ellps.E * ((self.lat_0.cos()).powi(2)) * ((az.cos()).powi(2))
                / self.ellps.E.mul_add(-self.ellps.E, 1.0);
        let big_b = 3.0
            * self.ellps.E
            * self.ellps.E
            * (1.0 - big_a)
            * self.lat_0.sin()
            * self.lat_0.cos()
            * az.cos()
            / self.ellps.E.mul_add(-self.ellps.E, 1.0);
        let big_d = c / self.n_1;
        let big_e = big_d
            - (big_a * (1.0 + big_a) * big_d.powi(3) / 6.0)
            - (big_b * 3.0f64.mul_add(big_a, 1.0) * big_d.powi(4) / 24.0);
        let big_f = 1.0 - (big_a * big_e * big_e / 2.0) - (big_b * big_e.powi(3) / 6.0);

        let psi =
            self.lat_0.sin().mul_add(big_e.cos(), self.lat_0.cos() * big_e.sin() * az.cos()).asin();

        let lon = self.lon_0 + (az.sin() * big_e.sin() / psi.cos()).asin();
        let lat = ((1.0 - (self.ellps.E * self.ellps.E * big_f * self.lat_0.sin() / psi.sin()))
            * psi.tan()
            / self.ellps.E.mul_add(-self.ellps.E, 1.0))
            .atan();

        let lon = lon.to_degrees();
        let lat = lat.to_degrees();

        (lon, lat)
    }
}
