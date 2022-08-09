//! ## Example
//! 
//! We can project the geographical coordinates to cartographic 
//! coordinates on a map with sepcified projection as follows:
//! 
//!```
//!# use mappers::{ellipsoids::WGS84, projections::LambertConformalConic, Projection, ProjectionError};
//!#
//!# fn main() -> Result<(), ProjectionError> {
//! // First, we define the projection
//! 
//! // We use LCC with reference longitude centered on France
//! // parallels set for Europe and WGS84 ellipsoid
//! let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, WGS84)?;
//! 
//! // Second, we define the coordinates of Mount Blanc
//! let (lon, lat) = (6.8651, 45.8326);
//! 
//! // Project the coordinates
//! let (x, y) = lcc.project(lon, lat)?;
//! 
//! // And print the result
//! println!("x: {}, y: {}", x, y); // x: 364836.4407792019, y: 5421073.726335758
//!# Ok(())
//!# }
//!```
//! 
//! We can also inversly project the cartographic coordinates 
//! to geographical coordinates:
//! 
//!```
//!# use mappers::{ellipsoids::WGS84, projections::LambertConformalConic, Projection, ProjectionError};
//!#
//!# fn main() -> Result<(), ProjectionError> {
//! // We again start with defining the projection
//! let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, WGS84)?;
//! 
//! // We take the previously projected coordinates
//! let (x, y) = (364836.4407792019, 5421073.726335758);
//! 
//! // Inversly project the coordinates
//! let (lon, lat) = lcc.inverse_project(x, y)?;
//! 
//! // And print the result
//! println!("lon: {}, lat: {}", lon, lat); // lon: 6.8651, lat: 45.83260000001716
//!# Ok(())
//!# }
//!```
//! Some projections are mathematically exactly inversible, and technically
//! geographical coordinates projected and inverse projected should be identical.
//! However, in practice limitations of floating-point arithmetics will 
//! introduce some errors along the way, as shown in the example above.
//! 

pub use errors::ProjectionError;

pub mod ellipsoids;
mod errors;
pub mod projections;

pub trait Projection {
    /// Function to project geographic coordinates
    /// on WGS84 ellipsoid to cartographic coordinates
    /// with previously specified LCC projection.
    fn project(&self, lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError> {
        let (x, y) = self.project_unchecked(lon, lat);

        if !x.is_finite() || !y.is_finite() {
            Err(ProjectionError::InverseProjectionImpossible(lon, lat))
        } else {
            Ok((x, y))
        }
    }

    /// Function to inversly project cartographic coordinates
    /// on specified LCC projection to geographic coordinates
    /// on WGS84 ellipsoid.
    fn inverse_project(&self, x: f64, y: f64) -> Result<(f64, f64), ProjectionError> {
        let (lon, lat) = self.inverse_project_unchecked(x, y);

        if !lon.is_finite() || !lat.is_finite() {
            Err(ProjectionError::InverseProjectionImpossible(x, y))
        } else {
            Ok((lon, lat))
        }
    }

    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64);
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64);
}
