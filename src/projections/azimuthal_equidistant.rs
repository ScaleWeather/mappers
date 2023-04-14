use crate::{Ellipsoid, Projection, ProjectionError};
use geographiclib_rs::{DirectGeodesic, Geodesic, InverseGeodesic};

/// The azimuthal equidistant projection is an azimuthal map projection.
/// It has the useful properties that all points on the map are at proportionally
/// correct distances from the center point, and that all points on the map are at the
/// correct azimuth (direction) from the center point. A useful application for this
/// type of projection is a polar projection which shows all meridians (lines of longitude) as straight,
/// with distances from the pole represented correctly [(Wikipedia, 2022)](https://en.wikipedia.org/wiki/Azimuthal_equidistant_projection).
///
/// This projection uses Geodesic computation (defined by [C. F. F. Karney (2013)](https://doi.org/10.1007/s00190-012-0578-z))
/// to compute distances and azimuths between projected point and origin. So it might be slower than some other projections.
///
/// Summary by [Snyder (1987)](https://pubs.er.usgs.gov/publication/pp1395):
///
/// - Azimuthal.
/// - Distances measured from the center are true.
/// - Distances not measured along radii from the center are not correct.
/// - The center of projection is the only point without distortion.
/// - Directions from the center are true (except on some oblique and equatorial ellipsoidal forms).
/// - Neither equal-area nor conformal.
/// - All meridians on the polar aspect, the central meridian on other aspects, and the Equator on the equatorial aspect are straight lines.
/// - Parallels on the polar projection are circles spaced at true intervals (equidistant for the sphere).
/// - The outer meridian of a hemisphere on the equatorial aspect (for the sphere) is a circle.
/// - All other meridians and parallels are complex curves.
/// - Not a perspective projection.
/// - Point opposite the center is shown as a circle (for the sphere) surrounding the map.
/// - Used in the polar aspect for world maps and maps of polar hemispheres.
/// - Used in the oblique aspect for atlas maps of continents and world maps for aviation and radio use.
/// - Known for many centuries in the polar aspect.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct AzimuthalEquidistant {
    lon_0: f64,
    lat_0: f64,
    geod: Geodesic,
}

impl AzimuthalEquidistant {
    /// AEQD projection constructor.
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
    /// - `ellps` - Reference [`Ellipsoid`].
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError::IncorrectParams`] with additional information when:
    ///
    /// - one or more longitudes are not within -180..180 range.
    /// - one or more latitudes are not within -90..90 range.
    /// - one or more arguments are not finite.
    pub fn new(ref_lon: f64, ref_lat: f64, ellps: Ellipsoid) -> Result<Self, ProjectionError> {
        if !ref_lon.is_finite() || !ref_lat.is_finite() {
            return Err(ProjectionError::IncorrectParams(
                "one of arguments is not finite",
            ));
        }

        if !(-180.0..180.0).contains(&ref_lon) {
            return Err(ProjectionError::IncorrectParams(
                "longitude must be between -180..180",
            ));
        }

        if !(-90.0..90.0).contains(&ref_lat) {
            return Err(ProjectionError::IncorrectParams(
                "latitude must be between -90..90",
            ));
        }

        Ok(AzimuthalEquidistant {
            lon_0: ref_lon,
            lat_0: ref_lat,
            geod: ellps.into(),
        })
    }
}

impl Projection for AzimuthalEquidistant {
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64) {
        let (s12, azi1, _, _) = self.geod.inverse(self.lat_0, self.lon_0, lat, lon);

        let x = s12 * azi1.to_radians().sin();
        let y = s12 * azi1.to_radians().cos();

        (x, y)
    }

    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        let azi1 = x.atan2(y).to_degrees();
        let s12 = x.hypot(y);

        let (lat, lon) = self.geod.direct(self.lat_0, self.lon_0, azi1, s12);

        (lon, lat)
    }
}
