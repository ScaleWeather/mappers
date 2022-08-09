//! Library with methods to do computations
//! of geographical projections.
//! Closely follows algorithms and instructions in:
//! <https://pubs.er.usgs.gov/publication/pp1395>

pub use errors::ProjectionError;

pub mod ellipsoids;
mod errors;
pub mod projections;

pub trait Projection {
    fn project(&self, lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError>;
    fn inverse_project(&self, x: f64, y: f64) -> Result<(f64, f64), ProjectionError>;
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64);
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64);
}
