use geo_types::Coord;
use kapta::view;
#[test]
fn render() {
    let h: u32 = 700;
    let w: u32 = 900;
    let zoom: u8 = 19;
    let center: Coord = (107., 17.).into();
    let _view = view::render(w, h, zoom, center);
    // dbg!(view);
}
