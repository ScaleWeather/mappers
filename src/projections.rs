//! Geographical projections implemented by the crate.

pub mod azimuthal_equidistant;
pub mod equidistant_cylindrical;
pub mod lambert_conformal_conic;
mod lon_lat;
pub mod modified_azimuthal_equidistant;
pub mod oblique_lon_lat;

pub use azimuthal_equidistant::AzimuthalEquidistant;
pub use equidistant_cylindrical::EquidistantCylindrical;
pub use lambert_conformal_conic::LambertConformalConic;
pub use lon_lat::LongitudeLatitude;
pub use modified_azimuthal_equidistant::ModifiedAzimuthalEquidistant;
pub use oblique_lon_lat::ObliqueLonLat;
