use thiserror::Error;

/// Errors related to geographic projection.
#[derive(Error, Debug)]
pub enum ProjectionError {
    #[error("Incorrect projection parameters: {0}")]
    IncorrectParams(&'static str),
}
