use float_cmp::assert_approx_eq;
use mappers::projections::ObliqueLonLat;
use mappers::Projection;
use proj::Proj;
use crate::GLOBAL_GEO_POINTS;
use crate::LOCAL_GEO_POINTS;
use crate::TestExtent;

pub(crate) fn basic_correctness() {
    // This projection does not depend on ellipsoid
    // Also, this projection produces degrees, and not meters, so it must be tested separately

    for pole_lon in (-180..180).step_by(60) {
        for pole_lat in (-90..90).step_by(60) {
            for central_lon in (-180..180).step_by(60) {
                let int_proj =
                    ObliqueLonLat::new(pole_lon as f64, pole_lat as f64, Some(central_lon as f64))
                        .unwrap();

                let proj_str = format!(
                    "+ellps=sphere +proj=ob_tran +o_proj=latlon +o_lat_p={} +o_lon_p={} +lon_0={}",
                    pole_lat, pole_lon, central_lon
                );

                test_points_with_proj(&int_proj, &proj_str, TestExtent::Global);
                test_points_with_proj(&int_proj, &proj_str, TestExtent::Local);
            }
        }
    }
}

fn test_points_with_proj(int_proj: &ObliqueLonLat, proj_str: &str, extent: TestExtent) {
    let ref_proj = Proj::new(proj_str).unwrap();

    let geo_points = match extent {
        TestExtent::Global => GLOBAL_GEO_POINTS,
        TestExtent::Local => LOCAL_GEO_POINTS,
    };

    for point in geo_points {
        // projection
        let (ref_x, ref_y) = ref_proj
            .project((point.0.to_radians(), point.1.to_radians()), false)
            .unwrap();

        let ref_lon = ref_x.to_degrees();
        let ref_lat = ref_y.to_degrees();

        let (tst_lon, tst_lat) = int_proj.project(point.0, point.1).unwrap();

        assert_approx_eq!(f64, ref_lon, tst_lon, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, ref_lat, tst_lat, epsilon = 0.000_000_1);

        // inverse projection
        let (ref_x, ref_y) = ref_proj
            .project((point.0.to_radians(), point.1.to_radians()), true)
            .unwrap();

        let ref_lon = ref_x.to_degrees();
        let ref_lat = ref_y.to_degrees();

        let (tst_lon, tst_lat) = int_proj.inverse_project(point.0, point.1).unwrap();

        assert_approx_eq!(f64, ref_lon, tst_lon, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, ref_lat, tst_lat, epsilon = 0.000_000_1);
    }
} 
