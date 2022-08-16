#![allow(non_snake_case)]
#![allow(clippy::excessive_precision)]

//! Reference ellipsoids that can be used with [`projections`](crate::projections).

use geographiclib_rs::Geodesic;

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

    pub fn clarke1866() -> Self {
        Ellipsoid::new(6378206.4, 294.9786982)
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

impl Into<Geodesic> for Ellipsoid {
    fn into(self) -> Geodesic {
        Geodesic::new(self.A, self.F)
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
