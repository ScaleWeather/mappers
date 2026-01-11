use float_cmp::assert_approx_eq;
use mappers::{
    projections::{AzimuthalEquidistant, LambertConformalConic, LongitudeLatitude},
    Ellipsoid, Projection,
};

const TOLERANCE: f64 = 1e-5;

#[test]
fn conversion_api_and_correctness() {
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

    let (lon, lat) = (25.0, 45.0);
    let (lcc_x, lcc_y) = lcc.project(lon, lat).unwrap();
    let (aeqd_x, aeqd_y) = aeqd.project(lon, lat).unwrap();

    let (test_lcc_x, test_lcc_y) = ll.pipe_to(&lcc).convert(lon, lat).unwrap();
    let (test_aeqd_x, test_aeqd_y) = lcc.pipe_to(&aeqd).convert(test_lcc_x, test_lcc_y).unwrap();
    let (test_lon, test_lat) = aeqd.pipe_to(&ll).convert(test_aeqd_x, test_aeqd_y).unwrap();

    assert_approx_eq!(f64, lcc_x, test_lcc_x, epsilon = TOLERANCE);
    assert_approx_eq!(f64, lcc_y, test_lcc_y, epsilon = TOLERANCE);
    assert_approx_eq!(f64, aeqd_x, test_aeqd_x, epsilon = TOLERANCE);
    assert_approx_eq!(f64, aeqd_y, test_aeqd_y, epsilon = TOLERANCE);
    assert_approx_eq!(f64, lon, test_lon, epsilon = TOLERANCE);
    assert_approx_eq!(f64, lat, test_lat, epsilon = TOLERANCE);
}
