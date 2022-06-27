//! Library with methods to do computations
//! of geographical projections.
//! Closely follows algorithms and instructions in:
//! <https://pubs.er.usgs.gov/publication/pp1395>

use errors::ProjectionError;

mod constants;
mod errors;
mod projections;

pub trait Projection {
    fn project(&self, lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError>;
    fn inverse_project(&self, x: f64, y: f64) -> Result<(f64, f64), ProjectionError>;
}

/// Front-facing struct of Lambert Conformal Conic projection.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct LambertConicConformal {
    lambda_0: f64,
    n: f64,
    big_f: f64,
    rho_0: f64,
}
