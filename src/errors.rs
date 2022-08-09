use thiserror::Error;

/// An interface for errors used within the crate and that the user may face.
#[derive(Error, Debug)]
pub enum ProjectionError {
    /// Returned when the projection definition parameters are incorrect.
    #[error("Incorrect projection parameters: {0}")]
    IncorrectParams(&'static str),

    /// Returned when projection of given values results in not finite results.
    #[error("Attempt to project lon: {0} lat: {1} results in not finite result")]
    ProjectionImpossible(f64, f64),

    /// Returned when inverse projection of given values results in not finite results.
    #[error("Attempt to inverse project x: {0} y: {1} results in not finite result")]
    InverseProjectionImpossible(f64, f64),
}
