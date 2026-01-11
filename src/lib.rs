#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]

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
//! coordinates on a map with one of implemented [`projections`] as follows:
//!
//!```
//!# use mappers::{Ellipsoid, projections::LambertConformalConic, Projection, ProjectionError};
//!#
//!# fn main() -> Result<(), ProjectionError> {
//! // First, we define the projection
//! // Projections are constructed with builders
//!
//! // We use LCC with reference longitude centered on France
//! // parallels set for Europe and WGS84 ellipsoid (defined by default)
//! let lcc = LambertConformalConic::builder()
//!     .ref_lonlat(30., 30.)
//!     .standard_parallels(30., 60.)
//!     .initialize_projection()?;
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
//! let lcc = LambertConformalConic::builder()
//!     .ref_lonlat(30., 30.)
//!     .standard_parallels(30., 60.)
//!     .initialize_projection()?;
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
//! ## `ConversionPipe`
//!
//! This crate also provides a struct [`ConversionPipe`] that allows for easy
//! conversion between two projections. It can be constructed directly from
//! [`Projection`] with [`pipe_to`](Projection::pipe_to) method or directly
//! with [`ConversionPipe::new()`].
//!
//! Before using it please read the documentation of [`ConversionPipe`].
//!
//! ### Example
//!
//!```
//!# use mappers::{Ellipsoid, Projection, ProjectionError, ConversionPipe};
//!# use mappers::projections::{LambertConformalConic, LongitudeLatitude};
//!# use float_cmp::assert_approx_eq;
//!#
//!# fn main() -> Result<(), ProjectionError> {
//! // We start by defining the source and target projections
//! // In this case we will use LCC and LongitudeLatitude
//! // to show how a normal projection can be done with ConversionPipe
//! let target_proj = LambertConformalConic::builder()
//!     .ref_lonlat(30., 30.)
//!     .standard_parallels(30., 60.)
//!     .initialize_projection()?;
//!
//! // LongitudeLatitude projection is an empty struct
//! let source_proj = LongitudeLatitude;
//!
//! let (lon, lat) = (6.8651, 45.8326);
//!
//! // Now we can convert to LCC and back to LongitudeLatitude
//! let (x, y) = source_proj.pipe_to(&target_proj).convert(lon, lat)?;
//! let (pipe_lon, pipe_lat) = target_proj.pipe_to(&source_proj).convert(x, y)?;
//!
//! // For simple cases the error remains small
//! // but it can quickly grow with more complex conversions
//! assert_approx_eq!(f64, lon, pipe_lon, epsilon = 1e-10);
//! assert_approx_eq!(f64, lat, pipe_lat, epsilon = 1e-10);
//!
//!# Ok(())
//!# }
//!```
//!
//! ## Tracing
//! Functions that are likely to be called in a complex chain of computations,
//! namely `project()`/`inverse_project()` (checked and unchecked) from [`Projection`] trait and
//! `convert()` (checked and unchecked) from [`ConversionPipe`], implement `instrument` macro from
//! `tracing` macro for easier debugging.
//!
//! This functionality must be enabled with `tracing` feature.
//!
//! These functions themselves don't emit any tracing messages so to get the information provided
//! by the `instrument` macro, tracing subscriber should be configured to show span events.
//! This can be achieved using, for example `.with_span_events(FmtSpan::FULL)`.

use std::fmt::Debug;

#[cfg(feature = "tracing")]
use tracing::instrument;

pub use ellipsoids::Ellipsoid;
pub use errors::ProjectionError;

mod ellipsoids;
mod errors;
pub mod projections;

/// An interface for all projections included in the crate.
///
/// This trait is kept relatively simple and the most basic version of
/// projection functions are implemented. Alternative functions for more complex
/// types should be implemented by the user.
///
/// Available projections are available in [`projections`] module documentation.
pub trait Projection: Debug + Send + Sync + Copy + Clone + PartialEq + PartialOrd {
    /// Function to project geographical coordinates (in degrees) to cartographical
    /// coordinates (in meters) on a map with specified projection.
    ///
    /// # Errors
    ///
    /// Returns [`ProjectionError::ProjectionImpossible`] when result of
    /// projection is not finite.
    #[inline]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
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
    #[inline]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
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

    /// Creates [`ConversionPipe`] from this projection to provided target projection.
    fn pipe_to<TARGET: Projection>(&self, target: &TARGET) -> ConversionPipe<Self, TARGET> {
        ConversionPipe::new(self, target)
    }
}

/// A struct that allows for easy conversion between two projections.
///
/// It can be constructed directly with the constructor or
/// from [`Projection`] with [`pipe_to`](Projection::pipe_to) method.
///
/// The implementation is very naive as it converts coordinates to longitude and latitude then projects them
/// to the target projection. Therefore projection and numerical errors are accumulated with every step and
/// long conversion chains are discouraged.
///
/// Main purpose of this struct is to allow creating generic conversion patterns independent of projections.
///
/// For usage see examples in [the main module](crate).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ConversionPipe<S: Projection, T: Projection> {
    source: S,
    target: T,
}

impl<S: Projection, T: Projection> ConversionPipe<S, T> {
    /// Creates a new conversion pipe from source to target projection.
    pub fn new(source: &S, target: &T) -> Self {
        Self {
            source: *source,
            target: *target,
        }
    }

    /// Reverse the direction of conversion.
    pub fn invert(&self) -> ConversionPipe<T, S> {
        ConversionPipe::new(&self.target, &self.source)
    }

    /// Converts the coordinates from source to target projection.
    ///
    /// # Errors
    ///
    /// This function uses checked projection methods and returns [`ProjectionError`] if any step
    /// emits non-finite values.
    #[inline]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
    pub fn convert(&self, x: f64, y: f64) -> Result<(f64, f64), ProjectionError> {
        let (lon, lat) = self.source.inverse_project(x, y)?;
        self.target.project(lon, lat)
    }

    /// Converts the coordinates from source to target projection without checking the result.
    #[inline]
    #[cfg_attr(feature = "tracing", instrument(level = "trace"))]
    pub fn convert_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        let (lon, lat) = self.source.inverse_project_unchecked(x, y);
        self.target.project_unchecked(lon, lat)
    }
}
