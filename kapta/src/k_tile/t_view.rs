use std::ops::Div;

use geo_types::Coord;

use crate::{
    consts::*,
    k_geo::{proj_to_tile, KCoord},
};

use super::{TileCollect, Tile, t_tile::{KTileCollect, KTile}};



#[derive(Debug, Default, Clone, PartialEq)]
pub struct TView {
    pub center: Coord,
    pub top_left: Coord,
    pub bottom_right: Coord,
    pub zoom: u8,
    pub array: Vec<(u8, i64, i64, f64)>,
}

impl TView {
    pub fn tile_size(len: u64) -> (f64, f64) {
        let tile_width = (BOUND_LON_3857 * 2.).div(len as f64);
        let tile_heigth = (BOUND_LAT_3857 * 2.).div(len as f64);
        (tile_width, tile_heigth)
    }

    pub fn load(center_proj: Coord, zoom: u8, width: u32, heigth: u32) -> Self {
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
            top_left: tl_proj_tile,
            bottom_right: br_proj_tile,
            zoom,
            array: array,
        }
    }

    pub fn to_vec_tile(&self, width: u32, heigth: u32, topleft_x: f64, topleft_y: f64) -> TileCollect {
        let mut vec_tile: Vec<Tile> = [].to_vec();
        for data in &self.array {
            let trans_x = (width / 2 - 128) as f64
                + (data.1 as f64 + 0.5 - self.center.x) * 255. + topleft_x;
            let trans_y = (heigth / 2 - 128) as f64
                + (data.2 as f64 + 0.5 - self.center.y) * 255. + topleft_y;
            let trans = format!("translate3d({}px, {}px, 0px)", trans_x, trans_y);
            let count = (2 as i64).pow(self.zoom.into());
            let url = format!(
                "https://tile.openstreetmap.org/{}/{}/{}.png",
                data.0,
                (data.1 as i64).rem_euclid(count),
                (data.2 as i64 % count),
            );
            vec_tile.push(Tile::new(&url, &trans))
        };
        
        TileCollect { collection: vec_tile }
    }

    pub fn to_k_tile_collect(&self, width: u32, heigth: u32, top_left: Coord) -> KTileCollect {
        let mut collection: Vec<KTile> = [].to_vec();
        for data in &self.array {
            collection.push(KTile { x: data.1, y: data.2, z: data.0 })
        }
        KTileCollect { collection, width, heigth, top_left, center: self.center }
    }

    pub fn similar_bound(&self, bound: (Coord, Coord)) -> bool {
        self.top_left.x as i64 == bound.0.x as i64
            && self.top_left.y as i64 == bound.0.y as i64
            && self.bottom_right.x as i64 == bound.1.x as i64
            && self.bottom_right.y as i64 == bound.1.y as i64
    }

    pub fn extent_bound(&self, bound: (Coord, Coord)) -> bool {
        (self.top_left.x as i64) > (bound.0.x as i64)
            || (self.top_left.y as i64) > (bound.0.y as i64)
            || (self.bottom_right.x as i64) < (bound.1.x as i64)
            || (self.bottom_right.y as i64) < (bound.1.y as i64)
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
