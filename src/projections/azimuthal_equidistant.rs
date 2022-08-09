use crate::ellipsoids::Ellipsoid;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct AzimuthalEquidistant {
    lambda_0: f64,
    n: f64,
    big_f: f64,
    rho_0: f64,
    ellps: Ellipsoid,
}
