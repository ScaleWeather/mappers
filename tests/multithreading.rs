use std::{sync::Arc, thread};

use float_cmp::assert_approx_eq;
use mappers::{
    projections::{AzimuthalEquidistant, LambertConformalConic, LongitudeLatitude},
    Ellipsoid, Projection,
};

#[test]
fn arc_interop() {
    let proj = AzimuthalEquidistant::builder()
        .ref_lonlat(30., 30.)
        .ellipsoid(Ellipsoid::WGS84)
        .initialize_projection()
        .unwrap();
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

        assert_approx_eq!(f64, map_coords.0, -398563.2994422894, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, map_coords.1, 1674853.7525355904, epsilon = 0.000_000_1);

        assert_approx_eq!(f64, geo_coords.0, 32.132000374279365, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, geo_coords.1, 32.68850065409422, epsilon = 0.000_000_1);
    }
}

#[test]
fn conversion_arc_interop() {
    let ll = LongitudeLatitude;
    let lcc = LambertConformalConic::builder()
        .ref_lonlat(30., 30.)
        .standard_parallels(30., 60.)
        .ellipsoid(Ellipsoid::WGS84)
        .initialize_projection()
        .unwrap();
    let aeqd = AzimuthalEquidistant::builder()
        .ref_lonlat(30., 30.)
        .ellipsoid(Ellipsoid::WGS84)
        .initialize_projection()
        .unwrap();

    let ll = Arc::new(ll);
    let lcc = Arc::new(lcc);
    let aeqd = Arc::new(aeqd);

    let mut handles = vec![];

    for _ in 0..10 {
        let ll = Arc::clone(&ll);
        let lcc = Arc::clone(&lcc);
        let aeqd = Arc::clone(&aeqd);

        let handle = thread::spawn(move || {
            let (lcc_x, lcc_y) = ll.pipe_to(&*lcc).convert(25.0, 45.0).unwrap();
            let (aeqd_x, aeqd_y) = lcc.pipe_to(&*aeqd).convert(lcc_x, lcc_y).unwrap();
            let coords = aeqd.pipe_to(&*ll).convert(aeqd_x, aeqd_y).unwrap();

            coords
        });
        handles.push(handle);
    }

    for handle in handles {
        let coords = handle.join().unwrap();

        assert_approx_eq!(f64, coords.0, 25.0, epsilon = 1e-8);
        assert_approx_eq!(f64, coords.1, 45.0, epsilon = 1e-8);
    }
}
