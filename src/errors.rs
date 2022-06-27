use thiserror::Error;

/// Errors related to geographic projection.
#[derive(Error, Debug)]
pub enum ProjectionError {
    #[error("Incorrect projection parameters: {0}")]
    IncorrectParams(&'static str),

    #[error("Attempt to project lon: {0} lat: {1} results in not finite result")]
    ProjectionImpossible(f64, f64),

    #[error("Attempt to inverse project x: {0} y: {1} results in not finite result")]
    InverseProjectionImpossible(f64, f64),
}
