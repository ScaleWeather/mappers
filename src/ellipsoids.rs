#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]

//! Reference ellipsoids that can be used with [`projections`](crate::projections).

use geographiclib_rs::Geodesic;

/// Ellipsoid struct that defines all values contained by reference ellipsoids.
///
/// Values for pre-defined ellipsoids are taken from the [EPSG Geodetic Parameter Dataset](https://epsg.org/),
/// [Map projections: A working manual (John P. Snyder, 1987)](https://pubs.er.usgs.gov/publication/pp1395) or
/// [Proj documentation](https://proj.org/usage/ellipsoids.html).
///
/// Because Rust consts currently do not support floating-point operations,
/// to maintain consistent precision across all targets pre-defined ellipsoids
/// are defined as functions. The overhead of calling these functions should be
/// negligible in most cases.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
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
    /// Ellipsoid constructor using semi-major axis and inverse flattening.
    pub fn new(semi_major_axis: f64, inverse_flattening: f64) -> Self {
        let I = inverse_flattening;
        let A = semi_major_axis;

        let F = 1.0 / I;
        let B = A - (A / I);
        let E = (1.0 - (B.powi(2) / A.powi(2))).sqrt();

        Ellipsoid { A, B, E, F }
    }

    /// Ellipsoid for a sphere with radius of 6,370,997.0 meters.
    pub fn sphere() -> Self {
        Ellipsoid {
            A: 6_370_997.0,
            B: 6_370_997.0,
            E: 0.0,
            F: 0.0,
        }
    }

    /// World Geodetic System 1984 (WGS84) ellipsoid (EPSG:7030).
    pub fn wgs84() -> Self {
        Ellipsoid::new(6_378_137.0, 298.257_223_563)
    }

    /// Geodetic Reference System 1980 (GRS 1980) ellipsoid (EPSG:7019).
    pub fn grs80() -> Self {
        Ellipsoid::new(6_378_137.0, 298.257_222_101)
    }

    /// World Geodetic System 1972 (WGS72) ellipsoid (EPSG:7043).
    pub fn wgs72() -> Self {
        Ellipsoid::new(6_378_135.0, 298.26)
    }

    /// Geodetic Reference System 1967 (GRS 1967) ellipsoid (EPSG:7036).
    pub fn grs67() -> Self {
        Ellipsoid::new(6_378_160.0, 298.247_167_427)
    }

    /// Airy 1830 ellipsoid (EPSG:7001).
    pub fn airy1830() -> Self {
        Ellipsoid::new(6_377_563.396, 299.324_964_6)
    }

    /// World Geodetic System 1966 (WGS66) ellipsoid.
    pub fn wgs66() -> Self {
        Ellipsoid::new(6_378_145.0, 298.25)
    }

    /// World Geodetic System 1960 (WGS60) ellipsoid.
    pub fn wgs60() -> Self {
        Ellipsoid::new(6_378_165.0, 298.3)
    }

    /// Clarke 1866 ellipsoid (EPSG:7008).
    pub fn clarke1866() -> Self {
        Ellipsoid::new(6_378_206.4, 294.978_698_2)
    }
}

impl From<Geodesic> for Ellipsoid {
    fn from(geod: Geodesic) -> Self {
        Ellipsoid {
            A: geod.a,
            B: geod._b,
            E: (1.0 - (geod._b.powi(2) / geod.a.powi(2))).sqrt(),
            F: geod.f,
        }
    }
}

impl From<Ellipsoid> for Geodesic {
    fn from(ellps: Ellipsoid) -> Geodesic {
        Geodesic::new(ellps.A, ellps.F)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;
    use geographiclib_rs::Geodesic;

    use crate::Ellipsoid;

    #[test]
    fn into_geod() {
        let ref_elps = Ellipsoid::wgs84();
        let ref_geod = Geodesic::wgs84();

        let con_geod: Geodesic = ref_elps.into();

        assert_approx_eq!(f64, ref_elps.A, con_geod.a);
        assert_approx_eq!(f64, ref_geod.a, con_geod.a);

        assert_approx_eq!(f64, ref_elps.B, con_geod._b);
        assert_approx_eq!(f64, ref_geod._b, con_geod._b);

        assert_approx_eq!(f64, ref_elps.F, con_geod.f);
        assert_approx_eq!(f64, ref_geod.f, con_geod.f);
    }

    #[test]
    fn from_geod() {
        let ref_elps = Ellipsoid::wgs84();
        let ref_geod = Geodesic::wgs84();

        let con_elps: Ellipsoid = ref_geod.into();

        assert_approx_eq!(f64, ref_elps.A, con_elps.A);
        assert_approx_eq!(f64, ref_geod.a, con_elps.A);

        assert_approx_eq!(f64, ref_elps.B, con_elps.B);
        assert_approx_eq!(f64, ref_geod._b, con_elps.B);

        assert_approx_eq!(f64, ref_elps.F, con_elps.F);
        assert_approx_eq!(f64, ref_geod.f, con_elps.F);
    }
}
