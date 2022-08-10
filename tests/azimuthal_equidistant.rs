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

    for (ellps, ellps_str) in ellps_list {
        let proj = AzimuthalEquidistant::new(18.0, 0.0, ellps).unwrap();

        println!("{}", ellps_str);

        internals::test_points_with_proj(
            &proj,
            &format!("+proj=aeqd +lat_0=0.0 +lon_0=18.0 +ellps={}", ellps_str),
        );
    }
}
