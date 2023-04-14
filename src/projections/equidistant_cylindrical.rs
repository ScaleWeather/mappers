use crate::ellipsoids::Ellipsoid;
use crate::errors::ProjectionError;
use crate::Projection;

/// The equirectangular projection (also called the **equidistant cylindrical** projection
/// or la carte parallélogrammatique projection), and which includes the special case
/// of the plate carrée projection (also called the geographic projection,
/// lat/lon projection, or plane chart), is a simple map projection attributed to
/// Marinus of Tyre, who Ptolemy claims invented the projection about AD 100.
/// [(Wikipedia, 2022)](https://en.wikipedia.org/wiki/Equirectangular_projection).
///
/// Summary by [Snyder (1987)](https://pubs.er.usgs.gov/publication/pp1395):
///
/// - Cylindrical.
/// - Neither equal-area nor conformal.
/// - Meridians and parallels are equidistant straight lines, intersecting at right angles.
/// - Poles shown as lines.
/// - Used for world or regional maps.
/// - Very simple construction.
/// - Used only in spherical form.
/// - Presented by Eratosthenes (B.C.) or Marinus (A.D. 100).
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct EquidistantCylindrical {
    ref_lat: f64,
    ref_lon: f64,
    std_par: f64,
}

impl EquidistantCylindrical {
    /// Equirectangular projection constructor.
    ///
    /// This is a trivial constructor that only checks input parameters.
    /// Ellipsoid is not definable as this projection is only defined for sphere.
    /// If standard parallel and reference longitude and latitude are 0, then
    /// this projection becomes *Lat-Lon* or *Plate Carrée* projection.
    ///
    /// # Arguments
    ///
    /// - `ref_lon`, `ref_lat` - Reference longitude and latitude. Point (0, 0) on the map will be at this coordinates.
    /// - `std_par` - Standard parallel (latitude) along which the scale is true.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError::IncorrectParams`] with additional information when:
    ///
    /// - one or more longitudes are not within -180..180 range.
    /// - one or more latitudes are not within -90..90 range.
    /// - one or more arguments are not finite.
    pub fn new(ref_lon: f64, ref_lat: f64, std_par: f64) -> Result<Self, ProjectionError> {
        if !ref_lon.is_finite() || !ref_lat.is_finite() || !std_par.is_finite() {
            return Err(ProjectionError::IncorrectParams(
                "one of arguments is not finite",
            ));
        }

        if !(-180.0..180.0).contains(&ref_lon) {
            return Err(ProjectionError::IncorrectParams(
                "longitude must be between -180..180",
            ));
        }

        if !(-90.0..90.0).contains(&ref_lat) || !(-90.0..90.0).contains(&std_par) {
            return Err(ProjectionError::IncorrectParams(
                "latitude must be between -90..90",
            ));
        }

        Ok(EquidistantCylindrical {
            ref_lat: ref_lat.to_radians(),
            ref_lon: ref_lon.to_radians(),
            std_par: std_par.to_radians(),
        })
    }
}

impl Projection for EquidistantCylindrical {
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64) {
        let lon = lon.to_radians();
        let lat = lat.to_radians();

        let r = Ellipsoid::SPHERE.A;

        let x = r * (lon - self.ref_lon) * self.std_par.cos();
        let y = r * (lat - self.ref_lat);

        (x, y)
    }

    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        let r = Ellipsoid::SPHERE.A;

        let lon = x / (r * self.std_par.cos()) + self.ref_lon;
        let lat = y / r + self.ref_lat;

        (lon.to_degrees(), lat.to_degrees())
    }
}
