use crate::test_points_with_proj;
use crate::TestExtent;
use mappers::projections::EquidistantCylindrical;

pub(crate) fn basic_correctness() {
    // This projection supports only spherical ellipsoid so must be tested separately

    for ref_lon in (-30..30).step_by(10) {
        for ref_lat in (-30..30).step_by(10) {
            for std_par in (-30..30).step_by(10) {
                let int_proj = EquidistantCylindrical::builder()
                    .ref_lonlat(ref_lon as f64, ref_lat as f64)
                    .standard_parallel(std_par as f64)
                    .initialize_projection()
                    .unwrap();

                let proj_str = format!(
                    "+proj=eqc +lon_0={} +lat_0={} +lat_ts={} +ellps=sphere",
                    ref_lon, ref_lat, std_par
                );

                test_points_with_proj(&int_proj, &proj_str, TestExtent::Global);
            }
        }
    }
}
