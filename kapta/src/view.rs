
use crate::k_tile::TView;
use geo_types::Coord;

pub fn render(width: u32, heigth: u32, zoom: u8, center: Coord) -> (u8, u32, u32, u32, u32, Vec<String>) {
   
    let t_view = TView::load(center, zoom.try_into().unwrap(), width, heigth);
    let trans_x = width/2;
    let trans_y = heigth/2;
    dbg!(&t_view);
    // let t_coord_tl = TView::top_left(center, zoom.try_into().unwrap(), width, heigth);
    // dbg!(t_coord_tl);

    (t_view.zoom, t_view.cx, t_view.cy, trans_x - t_view.xt, trans_y - t_view.yt, t_view.array)

}
