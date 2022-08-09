#![allow(non_snake_case)]

//! Reference ellipsoids that can be used with [`projections`](crate::projections).

/// Ellipsoid struct that defines all values contained by reference ellipsoids.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Ellipsoid {
    /// Ellipsoid semi-major axis
    pub A: f64,

    /// Ellipsoid semi-minor axis
    pub B: f64,

    /// Ellipsoid eccentricity
    pub E: f64,

    /// Ellipsoid Ramanujan's `h` parameter
    pub H: f64,
}

///WGS 84 ellipsoid. Axes lengths sourced from Wikipedia.
pub const WGS84: Ellipsoid = Ellipsoid {
    A: 6_378_137.0,

    #[allow(clippy::excessive_precision)]
    B: 6_356_752.314_245,

    #[allow(clippy::excessive_precision)]
    E: 0.081_819_190_842_965_558_441_157_725_155_790_103_599_429_130_554_199_218_75,

    #[allow(clippy::excessive_precision)]
    H: 0.001_679_220_386_397_858_7,
};
