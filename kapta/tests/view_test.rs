use kapta::{
    coords::{Coord, KaptaCoord, ProjCoord, Proj},
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

    let center_p3857 = ProjCoord {
        coord: Coord {
            x: 32084054.14618125,
            y: 5797226.819944519,
        },
        kind: Proj::EPSG3857,
        distance2: 0.0,
    };
    let view_new= KaptaView::new(center_p3857, topleft, width, height, zoom);
    dbg!(&view_new);

    let col = view_new.new_collection();
    dbg!(col);
}