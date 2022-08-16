use std::{sync::Arc, thread};

use float_cmp::assert_approx_eq;
use mappers::{projections::AzimuthalEquidistant, Ellipsoid, Projection};

#[test]
fn arc_interop() {
    let proj = AzimuthalEquidistant::new(30.0, 30.0, Ellipsoid::wgs84()).unwrap();
    let proj = Arc::new(proj);
    let mut handles = vec![];

    for _ in 0..10 {
        let proj = Arc::clone(&proj);
        let handle = thread::spawn(move || {
            let map_coords = proj.project(25.0, 45.0).unwrap();
            let geo_coords = proj.inverse_project(200_000.0, 300_000.0).unwrap();

            (map_coords, geo_coords)
        });
        handles.push(handle);
    }

    for handle in handles {
        let (map_coords, geo_coords) = handle.join().unwrap();

        assert_approx_eq!(f64, map_coords.0, -398563.2994422894);
        assert_approx_eq!(f64, map_coords.1, 1674853.7525355904);

        assert_approx_eq!(f64, geo_coords.0, 32.132000374279365);
        assert_approx_eq!(f64, geo_coords.1, 32.68850065409422);
    }
}
