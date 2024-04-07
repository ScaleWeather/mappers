use mappers::{projections::LambertConformalConic, Ellipsoid};

#[test]
fn test_constructor() {
    {
        let lcc = LambertConformalConic::new(2.0, 0.0, 0.0, 0.0, Ellipsoid::WGS84);
        assert!(lcc.is_err());
    }

    for std_par_1 in 1..90 {
        for std_par_2 in 1..90 {
            let lcc = LambertConformalConic::new(
                2.0,
                0.0,
                std_par_1 as f64,
                std_par_2 as f64,
                Ellipsoid::WGS84,
            );
            assert!(lcc.is_ok());
        }
    }

    for std_par_1 in -90..-1 {
        for std_par_2 in -90..-1 {
            let lcc = LambertConformalConic::new(
                2.0,
                0.0,
                std_par_1 as f64,
                std_par_2 as f64,
                Ellipsoid::WGS84,
            );
            assert!(lcc.is_ok());
        }
    }

    for std_par_1 in 1..90 {
        let std_par_2 = -std_par_1;
        let lcc = LambertConformalConic::new(
            2.0,
            0.0,
            std_par_1 as f64,
            std_par_2 as f64,
            Ellipsoid::WGS84,
        );
        assert!(lcc.is_err());
    }
}
