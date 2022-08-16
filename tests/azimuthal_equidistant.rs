use internals::TestExtent::Local;
use mappers::{
    ellipsoids::{GRS80, SPHERE, WGS60, WGS66, WGS72, WGS84},
    projections::AzimuthalEquidistant,
};
mod internals;

#[test]
fn project() {
    let ellps_list = [
        (WGS84, "WGS84"),
        (WGS60, "WGS60"),
        (WGS66, "WGS66"),
        (WGS72, "WGS72"),
        (GRS80, "GRS80"),
        (SPHERE, "sphere"),
    ];

    for (ellps, ellps_name) in ellps_list {
        let proj = AzimuthalEquidistant::new(30.0, 30.0, ellps).unwrap();

        println!("{}", ellps_name);

        internals::test_points_with_proj(
            &proj,
            &format!("+proj=aeqd +lon_0=30.0 +lat_0=30.0 +ellps={}", ellps_name),
            Local,
        );
    }
}
