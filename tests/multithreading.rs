use std::{sync::Arc, thread};

use float_cmp::assert_approx_eq;
use mappers::{projections::AzimuthalEquidistant, Ellipsoid, Projection};

#[test]
fn arc_interop() {
    let proj = AzimuthalEquidistant::new(30.0, 30.0, Ellipsoid::WGS84).unwrap();
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

#[cfg(feature = "multithreading")]
#[test]
fn parallelism_checked() {
    let proj = AzimuthalEquidistant::new(30.0, 30.0, Ellipsoid::WGS84).unwrap();

    let coords = vec![(25.0, 45.0); 100];
    let xy_points = vec![(200_000.0, 300_000.0); 100];

    let map_coords = proj.project_parallel(&coords).unwrap();
    let geo_coords = proj.inverse_project_parallel(&xy_points).unwrap();

    for map_coord in map_coords {
        assert_approx_eq!(f64, map_coord.0, -398563.2994422894, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, map_coord.1, 1674853.7525355904, epsilon = 0.000_000_1);
    }

    for geo_coord in geo_coords {
        assert_approx_eq!(f64, geo_coord.0, 32.132000374279365, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, geo_coord.1, 32.68850065409422, epsilon = 0.000_000_1);
    }
}

#[cfg(feature = "multithreading")]
#[test]
fn parallelism_unchecked() {
    let proj = AzimuthalEquidistant::new(30.0, 30.0, Ellipsoid::WGS84).unwrap();

    let coords = vec![(25.0, 45.0); 100];
    let xy_points = vec![(200_000.0, 300_000.0); 100];

    let map_coords = proj.project_parallel_unchecked(&coords);
    let geo_coords = proj.inverse_project_parallel_unchecked(&xy_points);

    for map_coord in map_coords {
        assert_approx_eq!(f64, map_coord.0, -398563.2994422894, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, map_coord.1, 1674853.7525355904, epsilon = 0.000_000_1);
    }

    for geo_coord in geo_coords {
        assert_approx_eq!(f64, geo_coord.0, 32.132000374279365, epsilon = 0.000_000_1);
        assert_approx_eq!(f64, geo_coord.1, 32.68850065409422, epsilon = 0.000_000_1);
    }
}

#[cfg(feature = "multithreading")]
#[test]
fn parallelism_arc_interop() {
    let proj = AzimuthalEquidistant::new(30.0, 30.0, Ellipsoid::WGS84).unwrap();
    let proj = Arc::new(proj);
    let mut handles = vec![];

    for _ in 0..10 {
        let proj = Arc::clone(&proj);
        let handle = thread::spawn(move || {
            let coords = vec![(25.0, 45.0); 100];
            let xy_points = vec![(200_000.0, 300_000.0); 100];

            let map_coords = proj.project_parallel_unchecked(&coords);
            let geo_coords = proj.inverse_project_parallel_unchecked(&xy_points);

            (map_coords, geo_coords)
        });
        handles.push(handle);
    }

    for handle in handles {
        let (map_coords, geo_coords) = handle.join().unwrap();

        for map_coord in map_coords {
            assert_approx_eq!(f64, map_coord.0, -398563.2994422894, epsilon = 0.000_000_1);
            assert_approx_eq!(f64, map_coord.1, 1674853.7525355904, epsilon = 0.000_000_1);
        }

        for geo_coord in geo_coords {
            assert_approx_eq!(f64, geo_coord.0, 32.132000374279365, epsilon = 0.000_000_1);
            assert_approx_eq!(f64, geo_coord.1, 32.68850065409422, epsilon = 0.000_000_1);
        }
    }
}

#[cfg(feature = "multithreading")]
#[ignore = "this test takes a long time to run"]
#[test]
fn parallelism_checked_long() {
    use mappers::projections::LambertConformalConic;
    use rand::{distributions::Uniform, Rng};

    let proj = LambertConformalConic::new(0.0, 30.0, 30.0, 60.0, Ellipsoid::WGS84).unwrap();
    let mut rng = rand::thread_rng();
    let lon_range = Uniform::new(-15.0, 15.0);
    let lat_range = Uniform::new(40.0, 50.0);

    let lons = (0..1000).map(|_| rng.sample(&lon_range)).collect::<Vec<_>>();
    let lons = std::iter::repeat(lons)
        .take(100_000)
        .flatten()
        .collect::<Vec<_>>();

    let lats = (0..1000).map(|_| rng.sample(&lat_range)).collect::<Vec<_>>();
    let lats = std::iter::repeat(lats)
        .take(100_000)
        .flatten()
        .collect::<Vec<_>>();

    let ref_coords: Vec<(f64, f64)> = lons.iter().copied().zip(lats.iter().copied()).collect();

    println!("Starting projection");

    let map_coords = proj.project_parallel(&ref_coords).unwrap();

    let geo_coords = proj.inverse_project_parallel(&map_coords).unwrap();

    for (ref_coord, geo_coord) in ref_coords.iter().zip(geo_coords.iter()) {
        assert_approx_eq!(f64, ref_coord.0, geo_coord.0, epsilon = 0.000_1);
        assert_approx_eq!(f64, ref_coord.1, geo_coord.1, epsilon = 0.000_1);
    }
}
