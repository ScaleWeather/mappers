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
}

///WGS 84 ellipsoid. Axes lengths sourced from Proj.
pub const WGS84: Ellipsoid = Ellipsoid {
    A: 6_378_137.0,

    #[allow(clippy::excessive_precision)]
    B: 6_356_752.314_245_179_295_539_855_957_031_25,

    #[allow(clippy::excessive_precision)]
    E: 0.081_819_190_842_621_569_714_765_428_216_196_596_622_467_041_015_625,
};

///WGS 72 ellipsoid. Axes lengths sourced from Proj.
pub const WGS72: Ellipsoid = Ellipsoid {
    A: 6_378_135.0,

    #[allow(clippy::excessive_precision)]
    B: 6_356_750.520_016_093_738_377_094_268_798_828_125,

    #[allow(clippy::excessive_precision)]
    E: 0.081_818_810_662_748_445_161_618_349_175_114_417_448_639_869_689_941_406_25,
};

///WGS 66 ellipsoid. Axes lengths sourced from Proj.
pub const WGS66: Ellipsoid = Ellipsoid {
    A: 6_378_145.0,

    #[allow(clippy::excessive_precision)]
    B: 6_356_759.769_488_683_901_727_199_554_443_359_375,

    #[allow(clippy::excessive_precision)]
    E: 0.081_820_179_996_059_783_089_634_720_454_341_731_965_541_839_599_609_375,
};

///WGS 60 ellipsoid. Axes lengths sourced from Proj.
pub const WGS60: Ellipsoid = Ellipsoid {
    A: 6_378_165.0,

    #[allow(clippy::excessive_precision)]
    B: 6_356_783.286_959_436_722_099_781_036_376_953_125,

    #[allow(clippy::excessive_precision)]
    E: 0.081_813_334_016_931_082_981_471_945_458_906_702_697_277_069_091_796_875,
};

///GRS 80 ellipsoid. Axes lengths sourced from Proj.
pub const GRS80: Ellipsoid = Ellipsoid {
    A: 6_378_137.0,

    #[allow(clippy::excessive_precision)]
    B: 6_356_752.314_140_356_145_799_160_003_662_109_375,

    #[allow(clippy::excessive_precision)]
    E: 0.081_819_191_042_815_139_769_395_216_262_637_404_724_955_558_776_855_468_75,
};

///Normal sphere. Axes lengths sourced from Proj.
pub const SPHERE: Ellipsoid = Ellipsoid {
    A: 6_370_997.0,
    B: 6_370_997.0,
    E: 0.0,
};
