//! Geographical projections implemented by the crate.

mod lambert_conformal_conic;
mod azimuthal_equidistant;
pub use lambert_conformal_conic::LambertConformalConic;
pub use azimuthal_equidistant::AzimuthalEquidistant;
