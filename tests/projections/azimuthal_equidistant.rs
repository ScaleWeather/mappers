use crate::internals::{
    self,
    TestExtent::{Global, Local},
    ELLIPSOIDS_TEST_SET,
};
use mappers::projections::AzimuthalEquidistant;

pub(crate) fn test_points_with_proj() {
    for (ellps, ellps_name) in ELLIPSOIDS_TEST_SET {
        let proj = AzimuthalEquidistant::new(30.0, 30.0, ellps).unwrap();

        println!("{}", ellps_name);

        internals::test_points_with_proj(
            &proj,
            &format!("+proj=aeqd +lon_0=30.0 +lat_0=30.0 +ellps={}", ellps_name),
            Global,
        );

        internals::test_points_with_proj(
            &proj,
            &format!("+proj=aeqd +lon_0=30.0 +lat_0=30.0 +ellps={}", ellps_name),
            Local,
        );
    }
}
