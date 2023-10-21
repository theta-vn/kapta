use crate::k_tile::TView;
use geo_types::Coord;

pub fn render(width: u32, heigth: u32, zoom: u8, center: Coord) -> TView {
    TView::load(center, zoom.try_into().unwrap(), width, heigth)
}
