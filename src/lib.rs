//! Library with methods to do computations
//! of geographical projections.
//! Closely follows algorithms and instructions in:
//! <https://pubs.er.usgs.gov/publication/pp1395>

pub use constants::Ellipsoid;
pub use projections::Projection;
pub use projections::lambert_conformal_conic::LambertConicConformal;
pub use errors::ProjectionError;

pub mod constants;
mod errors;
mod projections;



