use mappers::projections::{LambertConicConformal, Projection};

#[test]
fn test_projection() {
    let lcc = LambertConicConformal::new(18.0, 30.0, 60.0).unwrap();

    let (ref_lon, ref_lat) = (18.58973722443749, 54.41412855026378);

    let (x, y) = lcc.project(ref_lon, ref_lat);
    let (lon, lat) = lcc.inverse_project(x, y);

    assert!(lon - ref_lon < 0.000001);
    assert!(lat - ref_lat < 0.000001);
}