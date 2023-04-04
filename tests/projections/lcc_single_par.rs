use crate::internals::{
    self,
    TestExtent::{Global, Local},
    ELLIPSOIDS_TEST_SET,
};
use mappers::projections::LambertConformalConic;

pub(crate) fn test_points_with_proj() {
    for (ellps, ellps_name) in ELLIPSOIDS_TEST_SET {
        let proj = LambertConformalConic::new(30.0, 30.0, 40.0, 40.0, ellps).unwrap();

        println!("{}", ellps_name);

        internals::test_points_with_proj(
            &proj,
            &format!(
                "+proj=lcc +lat_1=40.0 +lat_2=40.0 +lon_0=30.0 +lat_0=30.0 +ellps={}",
                ellps_name
            ),
            Global,
        );

        internals::test_points_with_proj(
            &proj,
            &format!(
                "+proj=lcc +lat_1=40.0 +lat_2=40.0 +lon_0=30.0 +lat_0=30.0 +ellps={}",
                ellps_name
            ),
            Local,
        );
    }
}
