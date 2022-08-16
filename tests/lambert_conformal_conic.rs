use internals::TestExtent::Global;
use mappers::{
    ellipsoids::{GRS80, SPHERE, WGS60, WGS66, WGS72, WGS84},
    projections::LambertConformalConic,
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
        let proj = LambertConformalConic::new(30.0, 30.0, 30.0, 60.0, ellps).unwrap();

        println!("{}", ellps_name);

        internals::test_points_with_proj(
            &proj,
            &format!("+proj=lcc +lat_1=30.0 +lat_2=60.0 +lon_0=30.0 +lat_0=30.0 +ellps={}", ellps_name),
            Global,
        );
    }
}
