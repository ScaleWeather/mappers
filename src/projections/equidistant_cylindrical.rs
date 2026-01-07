use crate::ellipsoids::Ellipsoid;
use crate::errors::{
    ensure_finite, ensure_within_range, unpack_required_parameter, ProjectionError,
};
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

    r: f64,
    r_time_par_cos: f64,
}

impl EquidistantCylindrical {
    /// Initializes builder with default values.
    /// Projection parameters can be set with builder methods,
    /// see documentation of those methods to check which parmeters are required
    /// and default values for optional arguments.
    #[must_use] 
    pub fn builder() -> EquidistantCylindricalBuilder {
        EquidistantCylindricalBuilder::default()
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct EquidistantCylindricalBuilder {
    ref_lon: Option<f64>,
    ref_lat: Option<f64>,
    std_par: f64,
}

impl Default for EquidistantCylindricalBuilder {
    fn default() -> Self {
        Self {
            ref_lon: None,
            ref_lat: None,
            std_par: 0.0,
        }
    }
}

impl EquidistantCylindricalBuilder {
    /// *(required)* Sets reference longitude and latitude. Point (0, 0) on the map will be at this coordinates.
    pub fn ref_lonlat(&mut self, lon: f64, lat: f64) -> &mut Self {
        self.ref_lon = Some(lon);
        self.ref_lat = Some(lat);
        self
    }

    /// *(optional)* Sets reference standard parallel along which scale is true, defaults to `0.0`.
    pub fn standard_parallel(&mut self, std_par: f64) -> &mut Self {
        self.std_par = std_par;
        self
    }

    /// Equirectangular projection constructor.
    ///
    /// To reduce computational overhead of projection functions this
    /// constructor is non-trivial and tries to do as much projection computations as possible.
    /// Thus creating a new structure can involve a significant computational overhead.
    /// When projecting multiple coordinates only one instance of the structure should be created
    /// and copied/borrowed as needed.
    ///
    /// Ellipsoid is not definable as this projection is only defined for sphere.
    ///
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
    pub fn initialize_projection(&self) -> Result<EquidistantCylindrical, ProjectionError> {
        let ref_lon = unpack_required_parameter!(self, ref_lon);
        let ref_lat = unpack_required_parameter!(self, ref_lat);
        let std_par = self.std_par;

        ensure_finite!(ref_lon, ref_lat, std_par);
        ensure_within_range!(ref_lon, -180.0..180.0);
        ensure_within_range!(ref_lat, -90.0..90.0);
        ensure_within_range!(std_par, -90.0..90.0);

        let r = Ellipsoid::SPHERE.A;
        let r_time_par_cos = r * std_par.to_radians().cos();

        Ok(EquidistantCylindrical {
            ref_lat: ref_lat.to_radians(),
            ref_lon: ref_lon.to_radians(),
            std_par: std_par.to_radians(),

            r,
            r_time_par_cos,
        })
    }
}

impl Projection for EquidistantCylindrical {
    #[inline]
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64) {
        let lon = lon.to_radians();
        let lat = lat.to_radians();

        let x = self.r_time_par_cos * (lon - self.ref_lon);
        let y = self.r * (lat - self.ref_lat);

        (x, y)
    }

    #[inline]
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        let lon = (x / self.r_time_par_cos) + self.ref_lon;
        let lat = (y / self.r) + self.ref_lat;

        (lon.to_degrees(), lat.to_degrees())
    }
}
