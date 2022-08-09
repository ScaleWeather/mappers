use float_cmp::assert_approx_eq;
use mappers::{ellipsoids::WGS84, projections::LambertConicConformal, Projection};

#[test]
fn test_projection() {
    let lcc = LambertConicConformal::new(18.0, 0.0, 30.0, 60.0, WGS84).unwrap();

    let (ref_lon, ref_lat) = (18.58973722443749, 54.41412855026378);

    let (x, y) = lcc.project(ref_lon, ref_lat).unwrap();
    let (lon, lat) = lcc.inverse_project(x, y).unwrap();

    assert_approx_eq!(f64, lon, ref_lon, epsilon = 0.000_000_000_01);
    assert_approx_eq!(f64, lat, ref_lat, epsilon = 0.000_000_000_01);
}
 