use float_cmp::assert_approx_eq;
use mappers::{Ellipsoid, Projection};
use proj::Proj;

#[allow(unused)]
pub enum TestExtent {
    Global,
    Local,
}

pub static ELLIPSOIDS_TEST_SET: [(Ellipsoid, &str); 6] = [
    (Ellipsoid::WGS84, "WGS84"),
    (Ellipsoid::WGS72, "WGS72"),
    (Ellipsoid::WGS66, "WGS66"),
    (Ellipsoid::WGS60, "WGS60"),
    (Ellipsoid::GRS80, "GRS80"),
    (Ellipsoid::SPHERE, "sphere"),
];

static GLOBAL_GEO_POINTS: [(f64, f64); 8] = [
    (45.0, 45.0),
    (-45.0, 45.0),
    (45.0, -45.0),
    (-45.0, -45.0),
    (135.0, 45.0),
    (-135.0, 45.0),
    (135.0, -45.0),
    (-135.0, -45.0),
];

static LOCAL_GEO_POINTS: [(f64, f64); 8] = [
    (31.48, 31.26),
    (28.51, 31.26),
    (31.44, 28.72),
    (28.55, 28.72),
    (33.00, 32.50),
    (26.99, 32.50),
    (27.14, 27.42),
    (32.85, 27.42),
];

static MAP_POINTS: [(f64, f64); 8] = [
    (100_000.0, 100_000.0),
    (-100_000.0, 100_000.0),
    (100_000.0, -100_000.0),
    (-100_000.0, -100_000.0),
    (200_000.0, 200_000.0),
    (-200_000.0, 200_000.0),
    (200_000.0, -200_000.0),
    (-200_000.0, -200_000.0),
];

pub fn basic_correctness<F>(proj_constr: F, partial_proj_str: &str)
where
    F: Fn(Ellipsoid) -> Box<dyn Projection>,
{
    for (ellps, ellps_name) in ELLIPSOIDS_TEST_SET {
        let int_proj = proj_constr(ellps);

        test_points_with_proj(
            int_proj.as_ref(),
            &format!("{} +ellps={}", partial_proj_str, ellps_name),
            TestExtent::Global,
        );

        test_points_with_proj(
            int_proj.as_ref(),
            &format!("{} +ellps={}", partial_proj_str, ellps_name),
            TestExtent::Local,
        );
    }
}

pub fn test_points_with_proj(int_proj: &dyn Projection, proj_str: &str, extent: TestExtent) {
    let ref_proj = Proj::new(proj_str).unwrap();

    let geo_points = match extent {
        TestExtent::Global => GLOBAL_GEO_POINTS,
        TestExtent::Local => LOCAL_GEO_POINTS,
    };

    for point in geo_points {
        let (ref_x, ref_y) = ref_proj
            .project((point.0.to_radians(), point.1.to_radians()), false)
            .unwrap();

        let (tst_x, tst_y) = int_proj.project(point.0, point.1).unwrap();

        assert_approx_eq!(f64, ref_x, tst_x, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, ref_y, tst_y, epsilon = 0.000_000_1);
    }

    for point in MAP_POINTS {
        let (ref_lon, ref_lat) = ref_proj.project(point, true).unwrap();

        let (tst_lon, tst_lat) = int_proj.inverse_project(point.0, point.1).unwrap();

        assert_approx_eq!(f64, ref_lon.to_degrees(), tst_lon, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, ref_lat.to_degrees(), tst_lat, epsilon = 0.000_000_1);
    }
}
