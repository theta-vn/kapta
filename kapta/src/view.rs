use crate::{k_tile::TView, k_geo::KCoord};
use geo_types::Coord;

pub fn render(width: u32, heigth: u32, zoom: u8, center: Coord) -> TView {
    let center_3857 = KCoord::from(center).transformed(crate::k_geo::CRS::EPSG3857);
    TView::load(center_3857, zoom.try_into().unwrap(), width, heigth)
}
