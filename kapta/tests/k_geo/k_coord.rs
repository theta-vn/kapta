// use std::f64::consts::PI;

// use geo_types::coord;
use approx::{self, assert_relative_eq};
use geo_types::coord;
use kapta::{
    consts::{BOUND_LAT_3857, BOUND_LON_3857},
    k_geo::{KCoord, CRS},
};
// use proj::Proj;

#[test]
fn new() {
    let coord = KCoord::new(107_f64, 26_f64);
    dbg!(coord);

    let coord_3857 = coord.transformed(CRS::EPSG3857);
    dbg!(coord_3857);

    let coord_4326 = coord_3857.transformed(CRS::EPSG4326);
    dbg!(coord_4326);

    assert_relative_eq!(coord, coord_4326);
}

#[test]
fn proj() {
    let coord = KCoord::new(0_f64, 0_f64);
    let proj = coord.to_proj_coord();
    let c = coord! { x: BOUND_LON_3857, y: BOUND_LAT_3857 };
    dbg!(proj);
    assert_relative_eq!(proj, c);

    let coord = KCoord::new(107_f64, 17_f64);
    let proj = coord.to_proj_coord();
    dbg!(proj);

    // let re = coord.to_tile_coord(2);
    // dbg!(re);
}
