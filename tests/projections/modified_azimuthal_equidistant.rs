use float_cmp::assert_approx_eq;
use mappers::{projections::ModifiedAzimuthalEquidistant, Ellipsoid, Projection};

pub(crate) fn test_points_with_proj() {
    // This projection has to be tested with numerical example provided in Snyder
    // as it is not implemented in Proj
    let proj =
        ModifiedAzimuthalEquidistant::new(145.741_658_9, 15.184_911_94, Ellipsoid::CLARKE1866)
            .unwrap();

    let (x, y) = proj.project(145.793_030_0, 15.246_525_83).unwrap();

    let ref_x = 34_176.20 - 28_657.52;
    let ref_y = 74_017.88 - 67_199.99;

    // Because the numerical example in Snyder gives
    // very low precision the epsilon must be big
    assert_approx_eq!(f64, x, ref_x, epsilon = 0.01);
    assert_approx_eq!(f64, y, ref_y, epsilon = 0.01);

    let (lon, lat) = proj.inverse_project(ref_x, ref_y).unwrap();

    let ref_lon = 145.793_030_0;
    let ref_lat = 15.246_525_8;

    // Because the numerical example in Snyder gives
    // very low precision the epsilon must be big
    assert_approx_eq!(f64, lon, ref_lon, epsilon = 0.000_000_1);
    assert_approx_eq!(f64, lat, ref_lat, epsilon = 0.000_000_1);
}
