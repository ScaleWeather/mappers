use mappers::{
    projections::{AzimuthalEquidistant, LambertConformalConic},
    Projection,
};

mod internals;
mod projections;

#[test]
fn azimuthal_equidistant() {
    let proj_constr = |ellps| {
        Box::new(AzimuthalEquidistant::new(30.0, 30.0, ellps).unwrap()) as Box<dyn Projection>
    };
    let partial_proj = "+proj=aeqd +lon_0=30.0 +lat_0=30.0";

    internals::basic_correctness(proj_constr, partial_proj);
}

#[test]
fn lambert_conformal_conic() {
    let proj_constr = |ellps| {
        Box::new(LambertConformalConic::new(30.0, 30.0, 30.0, 60.0, ellps).unwrap())
            as Box<dyn Projection>
    };
    let partial_proj = "+proj=lcc +lat_1=30.0 +lat_2=60.0 +lon_0=30.0 +lat_0=30.0";

    internals::basic_correctness(proj_constr, partial_proj);
}

#[test]
fn lcc_single_par() {
    let proj_constr = |ellps| {
        Box::new(LambertConformalConic::new(30.0, 30.0, 40.0, 40.0, ellps).unwrap())
            as Box<dyn Projection>
    };
    let partial_proj = "+proj=lcc +lat_1=40.0 +lat_2=40.0 +lon_0=30.0 +lat_0=30.0";

    internals::basic_correctness(proj_constr, partial_proj);
}

#[test]
fn modified_azimuthal_equidistant() {
    projections::modified_azimuthal_equidistant::basic_correctness();
}
