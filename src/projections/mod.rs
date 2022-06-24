pub mod lambert_conformal_conic;

pub trait Projection {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64);
    fn inverse_project(&self, x: f64, y: f64) -> (f64, f64);
}


/// Front-facing struct of Lambert Conformal Conic projection.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct LambertConicConformal {
    lambda_0: f64,
    n: f64,
    big_f: f64,
    rho_0: f64,
}
