use crate::{k_geo::KCoord, k_tile::TView};
use geo_types::Coord;

pub fn render(width: u32, heigth: u32, zoom: u8, center: Coord) -> TView {
    let center_3857 = KCoord::from(center).transformed(crate::k_geo::CRS::EPSG3857);
    let center_proj = center_3857.to_proj_coord();
    TView::load(center_proj, zoom.try_into().unwrap(), width, heigth)
}
