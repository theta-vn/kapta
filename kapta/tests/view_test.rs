use geo_types::Coord;
use kapta::k_view::KView;
#[test]
fn render() {
    let height: u32 = 700;
    let width: u32 = 900;
    let zoom: u8 = 3;
    let center: Coord = (107., 17.).into();
    let topleft: Coord = (80., 80.).into();    
    
    let kview = KView::new(center, topleft, width, height, zoom);
    dbg!(&kview);
    let kcollection = kview.new_collection();
    dbg!(&kcollection);
    let vec_img = kview.to_img(kcollection);
    dbg!(vec_img);


}
