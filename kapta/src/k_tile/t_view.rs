use std::ops::Div;

use geo_types::Coord;

use crate::{
    consts::*,
    k_geo::{proj_to_tile, KCoord},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TView {
    pub center: Coord,
    pub zoom: u8,
    pub array: Vec<(u8, i64, i64, f64)>,
}

impl TView {
    pub fn tile_size(len: u64) -> (f64, f64) {
        let tile_width = (BOUND_LON_3857 * 2.).div(len as f64);
        let tile_heigth = (BOUND_LAT_3857 * 2.).div(len as f64);
        (tile_width, tile_heigth)
    }

    pub fn load(center: KCoord, zoom: u8, width: u32, heigth: u32) -> Self {
        let center_proj = center.to_proj_coord();
        let center_proj_tile = proj_to_tile(center_proj, zoom);

        let (tl_proj_tile, br_proj_tile) = bound_rec_tile(center_proj, zoom, width, heigth);

        let length_x = br_proj_tile.x as i64 - tl_proj_tile.x as i64 + 1;
        let length_y = br_proj_tile.y as i64 - tl_proj_tile.y as i64 + 1;

        let mut array: Vec<(u8, i64, i64, f64)> = [].to_vec();

        let length_tile = (2 as i64).pow(zoom.into());

        for m in 0..length_y {
            for n in 0..length_x {
                let x = tl_proj_tile.x as i64 + n;
                let y = tl_proj_tile.y as i64 + m;
                if 0 <= y && y < length_tile {
                    array.push((
                        zoom,
                        x,
                        y,
                        (x as f64 + 0.5 - center_proj_tile.x).powf(2.)
                            + (y as f64 + 0.5 - center_proj_tile.y).powf(2.),
                    ));
                }
            }
        }

        array.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

        Self {
            center: center_proj_tile,
            zoom,
            array: array,
        }
    }
}

pub fn bound_rec_tile(center_proj: Coord, zoom: u8, width: u32, heigth: u32) -> (Coord, Coord) {
    let center_tile = proj_to_tile(center_proj, zoom);

    let dx = (width / 2 - 1) as f64 / TILE as f64;
    let dy = (heigth / 2 - 1) as f64 / TILE as f64;

    let tl_tile = Coord {
        x: (center_tile.x - dx).floor(),
        y: center_tile.y - dy,
    };
    let br_tile = Coord {
        x: center_tile.x + dx,
        y: center_tile.y + dy,
    };
    (tl_tile, br_tile)
}
