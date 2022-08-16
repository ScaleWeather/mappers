use crate::{Ellipsoid, Projection, ProjectionError};
use geographiclib_rs::{DirectGeodesic, Geodesic, InverseGeodesic};

#[derive(Copy, Clone, Debug)]
pub struct AzimuthalEquidistant {
    lon_0: f64,
    lat_0: f64,
    geod: Geodesic,
}

impl AzimuthalEquidistant {
    pub fn new(ref_lon: f64, ref_lat: f64, ellps: Ellipsoid) -> Result<Self, ProjectionError> {
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

        if !ref_lon.is_finite() || !ref_lat.is_finite() {
            return Err(ProjectionError::IncorrectParams(
                "one of arguments is not finite",
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
