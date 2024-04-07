use crate::Projection;

/// This is a trivial projection that does not project anything.
/// Its purpose is to be used in generic uses of `ConversionPipe` where
/// source or target uses geographical coordinates.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LongitudeLatitude;

impl Projection for LongitudeLatitude {
    #[inline(always)]
    fn project_unchecked(&self, lon: f64, lat: f64) -> (f64, f64) {
        (lon, lat)
    }

    #[inline(always)]
    fn inverse_project_unchecked(&self, x: f64, y: f64) -> (f64, f64) {
        (x, y)
    }
}
