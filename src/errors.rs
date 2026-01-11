use thiserror::Error;

/// An interface for errors used within the crate and that the user may face.
#[derive(Error, Debug)]
pub enum ProjectionError {
    /// Returned when given parameter is not finite.
    #[error("Projection parameter {0} is not finite")]
    ParamNotFinite(&'static str),

    /// Returned when required parameter is missing.
    #[error("Parameter {0} must be definied")]
    ParamRequired(&'static str),

    /// Returned when parameter is too big or too small.
    #[error("Parameter {0} is out of required range {1}..{2}")]
    ParamOutOfRange(&'static str, f64, f64),

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

macro_rules! unpack_required_parameter {
    ($self:ident, $param: ident) => {
        $self
            .$param
            .ok_or(ProjectionError::ParamRequired(stringify!($param)))?
    };
}
pub(crate) use unpack_required_parameter;

macro_rules! ensure_finite {
    ($param:ident) => {
        if !$param.is_finite() {
            return Err(ProjectionError::ParamNotFinite(stringify!($param)));
        }
    };

    ($first:ident, $($rest:ident),+$(,)?) => {
        ensure_finite!($first);
        ensure_finite!($($rest),+);
    };
}
pub(crate) use ensure_finite;

macro_rules! ensure_within_range {
    ($param:ident, $range:expr) => {
        if !($range).contains(&$param) {
            return Err(ProjectionError::ParamOutOfRange(
                stringify!($param),
                $range.start,
                $range.end,
            ));
        }
    };
}
pub(crate) use ensure_within_range;
