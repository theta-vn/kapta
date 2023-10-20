// use std::f64::consts::PI;

// use geo_types::coord;
use approx::{self, assert_relative_eq};
use kapta::k_geo::{KCoord, CRS};
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

// #[test]
// fn convert() {
//     use geo_types::{point, Point};
//     use proj::Transform;

//     let point = point!(x: -107f64, y: -17f64);
//     let c = coord! { x: 10., y: 20. };
//     let a = c
//         .transformed_crs_to_crs("EPSG:4326", "EPSG:3857")
//         .unwrap();

//     dbg!(&a);

//     let b = a
//         .transformed_crs_to_crs( "EPSG:3857", "EPSG:4326")
//         .unwrap();
//     dbg!(&b);
//     // point!(x: -4064052.0f32, y: -7223650.5f32)

// }
