use kapta::{
    coords::{Coord, KaptaCoord, ProjCoord},
    views::KaptaView,
};
#[test]
fn render() {
    let height: u32 = 700;
    let width: u32 = 900;
    let zoom: u8 = 3;
    let center = KaptaCoord::new(107., 17.);
    let topleft: Coord = (80., 80.).into();
    let center_proj = ProjCoord::from(center);

    let kview = KaptaView::new(center_proj, topleft, width, height, zoom);
    dbg!(&kview);
    let kcollection = kview.new_collection();
    dbg!(&kcollection);
    let vec_img = kview.to_img(kcollection);
    dbg!(vec_img);
}
