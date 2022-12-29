use crate::internals::{
    self,
    TestExtent::{Global, Local},
};
use mappers::{projections::LambertConformalConic, Ellipsoid};

pub(crate) fn test_points_with_proj() {
    let ellps_list: [(Ellipsoid, &str); 6] = [
        (Ellipsoid::wgs84(), "WGS84"),
        (Ellipsoid::wgs72(), "WGS72"),
        (Ellipsoid::wgs66(), "WGS66"),
        (Ellipsoid::wgs60(), "WGS60"),
        (Ellipsoid::grs80(), "GRS80"),
        (Ellipsoid::sphere(), "sphere"),
    ];

    for (ellps, ellps_name) in ellps_list {
        let proj = LambertConformalConic::new(30.0, 30.0, 30.0, 60.0, ellps).unwrap();

        println!("{}", ellps_name);

        internals::test_points_with_proj(
            &proj,
            &format!(
                "+proj=lcc +lat_1=30.0 +lat_2=60.0 +lon_0=30.0 +lat_0=30.0 +ellps={}",
                ellps_name
            ),
            Global,
        );

        internals::test_points_with_proj(
            &proj,
            &format!(
                "+proj=lcc +lat_1=30.0 +lat_2=60.0 +lon_0=30.0 +lat_0=30.0 +ellps={}",
                ellps_name
            ),
            Local,
        );
    }
}