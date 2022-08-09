use crate::ProjectionError;

pub trait Projection {
    fn project(&self, lon: f64, lat: f64) -> Result<(f64, f64), ProjectionError>;
    fn inverse_project(&self, x: f64, y: f64) -> Result<(f64, f64), ProjectionError>;
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64);
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64);
}

pub(crate) mod lambert_conformal_conic;
