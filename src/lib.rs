//! Pure Rust geographical projections library. Similar to `Proj` in
//! basic functionality but allows for a use in concurrent contexts.
//!
//! Projections' implementations closely follow algorithms and instructions provided in:
//! [Map projections: A working manual (John P. Snyder, 1987)](https://pubs.er.usgs.gov/publication/pp1395)
//!
//! **This crate in very early stages of development. If you are interested
//! in contributing do not hesitate to contact me on Github.**
//!
//! ## Usage example
//!
//! We can project the geographical coordinates to cartographic
//! coordinates on a map with sepcified projection as follows:
//!
//!```
//!# use mappers::{Ellipsoid, projections::LambertConformalConic, Projection, ProjectionError};
//!#
//!# fn main() -> Result<(), ProjectionError> {
//! // First, we define the projection
//!
//! // We use LCC with reference longitude centered on France
//! // parallels set for Europe and WGS84 ellipsoid
//! let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, Ellipsoid::WGS84)?;
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
//!# use mappers::{Ellipsoid, projections::LambertConformalConic, Projection, ProjectionError};
//!#
//!# fn main() -> Result<(), ProjectionError> {
//! // We again start with defining the projection
//! let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, Ellipsoid::WGS84)?;
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
//! 
//! Some projections are mathematically exactly inversible, and technically
//! geographical coordinates projected and inverse projected should be identical.
//! However, in practice limitations of floating-point arithmetics will
//! introduce some errors along the way, as shown in the example above.
//! 
//! ## Multithreading
//! 
//! For projecting multiple coordinates at once, the crate provides `_parallel`
//! functions that are available in a (default) `multithreading` feature. These functions
//! use `rayon` crate to parallelize the projection process. They are provided 
//! mainly for convenience, as they are not much different than calling 
//! `.par_iter()` on a slice of coordinates and mapping the projection function over it.
//! 
//!```
//!# use mappers::{Ellipsoid, projections::LambertConformalConic, Projection, ProjectionError};
//!#
//!# fn main() -> Result<(), ProjectionError> {
//! let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, Ellipsoid::WGS84)?;
//!
//! // Parallel functions use slices of tuples as input and output
//! let geographic_coordinates = vec![(6.8651, 45.8326); 10];
//!
//! let map_coordinates = lcc.project_parallel(&geographic_coordinates)?;
//! let inversed_coordinates = lcc.inverse_project_parallel(&map_coordinates)?;
//!
//!# Ok(())
//!# }
//!```

use dyn_clone::DynClone;
use std::fmt::Debug;

pub use ellipsoids::Ellipsoid;
pub use errors::ProjectionError;

mod ellipsoids;
mod errors;
pub mod projections;

/// An interface for all projections included in the crate.
///
/// This trait is kept as simple as possible and the most basic version of
/// projection functions are implemented. Alternative functions for more complex
/// types should be implemented by the user.
pub trait Projection: Debug + DynClone + Send + Sync {
    /// Function to project geographical coordinates (in degrees) to cartographical
    /// coordinates (in meters) on a map with specified projection.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError::ProjectionImpossible`] when result of
    /// projection is not finite.
    fn project(&self, lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError> {
        let (x, y) = self.project_unchecked(lon, lat);

        if !x.is_finite() || !y.is_finite() {
            Err(ProjectionError::ProjectionImpossible(lon, lat))
        } else {
            Ok((x, y))
        }
    }

    /// Function to inversly project cartographical
    /// coordinates (in meters) to geographical coordinates (in degrees)
    /// on a map with specified projection.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError::InverseProjectionImpossible`] when result of
    /// inverse projection is not finite.
    fn inverse_project(&self, x: f64, y: f64) -> Result<(f64, f64), ProjectionError> {
        let (lon, lat) = self.inverse_project_unchecked(x, y);

        if !lon.is_finite() || !lat.is_finite() {
            Err(ProjectionError::InverseProjectionImpossible(x, y))
        } else {
            Ok((lon, lat))
        }
    }

    /// Same as [`Projection::project()`] but does not check the result.
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64);

    /// Same as [`Projection::inverse_project()`] but does not check the result.
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64);

    /// Function analogous to [`Projection::project()`] for projecting
    /// multiple coordinates at once using multithreading.
    #[cfg(feature = "multithreading")]
    fn project_parallel(&self, coords: &[(f64, f64)]) -> Result<Vec<(f64, f64)>, ProjectionError> {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let xy_points = coords
            .par_iter()
            .map(|(lon, lat)| self.project(*lon, *lat))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(xy_points)
    }

    /// Function analogous to [`Projection::inverse_project()`] for projecting
    /// multiple coordinates at once using multithreading.
    #[cfg(feature = "multithreading")]
    fn inverse_project_parallel(
        &self,
        xy_points: &[(f64, f64)],
    ) -> Result<Vec<(f64, f64)>, ProjectionError> {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let coords = xy_points
            .par_iter()
            .map(|(x, y)| self.inverse_project(*x, *y))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(coords)
    }

    /// Same as [`Projection::project_parallel()`] but does not check the result.
    #[cfg(feature = "multithreading")]
    fn project_parallel_unchecked(&self, coords: &[(f64, f64)]) -> Vec<(f64, f64)> {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let xy_points = coords
            .par_iter()
            .map(|(lon, lat)| self.project_unchecked(*lon, *lat))
            .collect::<Vec<_>>();

        xy_points
    }

    /// Same as [`Projection::inverse_project_parallel()`] but does not check the result.
    #[cfg(feature = "multithreading")]
    fn inverse_project_parallel_unchecked(&self, xy_points: &[(f64, f64)]) -> Vec<(f64, f64)> {
        use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

        let coords = xy_points
            .par_iter()
            .map(|(x, y)| self.inverse_project_unchecked(*x, *y))
            .collect::<Vec<_>>();

        coords
    }
}

dyn_clone::clone_trait_object!(Projection);
