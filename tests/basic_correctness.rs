mod internals;
mod projections;

#[test]
fn azimuthal_equidistant() {
    projections::azimuthal_equidistant::test_points_with_proj();
}

#[test]
fn lambert_conformal_conic() {
    projections::lambert_conformal_conic::test_points_with_proj();
}

#[test]
fn modified_azimuthal_equidistant() {
    projections::modified_azimuthal_equidistant::test_points_with_proj();
}
