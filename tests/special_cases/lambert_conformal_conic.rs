use mappers::{Ellipsoid, ProjectionError, projections::LambertConformalConic};

#[test]
fn test_constructor() {
    let mut partial_builder = LambertConformalConic::builder();
    partial_builder
        .ref_lonlat(2., 0.)
        .ellipsoid(Ellipsoid::WGS84);

    {
        let lcc = partial_builder
            .standard_parallels(0., 0.)
            .initialize_projection()
            .unwrap_err();
        assert!(std::matches!(lcc, ProjectionError::IncorrectParams { .. }));
    }

    for std_par_1 in 1..90 {
        for std_par_2 in 1..90 {
            let lcc = partial_builder
                .standard_parallels(std_par_1 as f64, std_par_2 as f64)
                .initialize_projection();
            assert!(lcc.is_ok());
        }
    }

    for std_par_1 in -90..-1 {
        for std_par_2 in -90..-1 {
            let lcc = partial_builder
                .standard_parallels(std_par_1 as f64, std_par_2 as f64)
                .initialize_projection();
            assert!(lcc.is_ok());
        }
    }

    for std_par_1 in 1..90 {
        let std_par_2 = -std_par_1;
        let lcc = partial_builder
            .standard_parallels(std_par_1 as f64, std_par_2 as f64)
            .initialize_projection()
            .unwrap_err();
        assert!(std::matches!(lcc, ProjectionError::IncorrectParams { .. }));
    }
}
