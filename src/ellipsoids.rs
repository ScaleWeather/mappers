#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]

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

    /// Ellipsoid flattening
    pub F: f64,
}

impl Ellipsoid {
    pub fn new(semi_major_axis: f64, inverse_flattening: f64) -> Self {
        let I = inverse_flattening;
        let A = semi_major_axis;

        let F = 1.0 / I;
        let B = A - (A / I);
        let E = (1.0 - (B.powi(2) / A.powi(2))).sqrt();

        Ellipsoid { A, B, E, F }
    }

    pub fn sphere() -> Self {
        Ellipsoid {
            A: 6_370_997.0,
            B: 6_370_997.0,
            E: 0.0,
            F: 0.0,
        }
    }

    pub fn wgs84() -> Self {
        Ellipsoid::new(6378137.0, 298.257223563)
    }

    pub fn grs80() -> Self {
        Ellipsoid::new(6378137.0, 298.257222101)
    }

    pub fn wgs72() -> Self {
        Ellipsoid::new(6378135.0, 298.26)
    }

    pub fn grs67() -> Self {
        Ellipsoid::new(6378160.0, 298.247167427)
    }

    pub fn airy1830() -> Self {
        Ellipsoid::new(6377563.396, 299.3249646)
    }

    pub fn wgs66() -> Self {
        Ellipsoid::new(6378145.0, 298.25)
    }

    pub fn wgs60() -> Self {
        Ellipsoid::new(6378165.0, 298.3)
    }
}
