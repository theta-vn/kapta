use geo_types::{Point, Coord};
use kapta::view;
#[test]
fn render() {
    
    let h: u32 = 700;
    let w: u32 = 900;    
    let zoom: u8 = 3;
    // let count = i32::pow(2, zoom);
    let center: Coord = (107., 17.).into();
    let _view = view::render(w, h, zoom, center);
    // dbg!(view);
}
