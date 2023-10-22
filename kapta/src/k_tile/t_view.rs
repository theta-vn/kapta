use std::{ops::Div, usize};

use geo_types::Coord;

use crate::{
    consts::*,
    k_geo::KCoord,
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
        let center_proj = center.to_tile_coord(zoom);

        // let length_tile = (2 as u64).pow(zoom.into()) as f64;

        let dx = ((width / 2 - 1) as f64 / TILE as f64).ceil();
        let dy = ((heigth / 2 - 1) as f64 / TILE as f64).ceil();
        dbg!(center_proj, dx, dy);
        let tl_proj = Coord {
            x: center_proj.x.floor() - dx,
            y: center_proj.y.floor() - dy,
        }; //translate(center_proj, -dx, -dy);
        let br_proj = Coord {
            x: center_proj.x.floor() + dx,
            y: center_proj.y.floor() + dy,
        }; //translate(center_proj, dx, dy);
        dbg!(center_proj, tl_proj, br_proj);
        // let tl_coord = tl_kc_3876.to_tile_coord(zoom);
        // let br_coord = br_kc_3876.to_tile_coord(zoom);
        // dbg!(center, tl_coord, br_coord);
        let length_x = br_proj.x as i64 - tl_proj.x as i64 + 1;
        let length_y = br_proj.y as i64 - tl_proj.y as i64 + 1;
        // dbg!(length_x, length_y);
        let mut array: Vec<(u8, i64, i64, f64)> = [].to_vec();
        // // let mut array_string: Vec<String> = [].to_vec();
        let length_tile = (2 as i64).pow(zoom.into());

        for m in 0..length_y {
            for n in 0..length_x {
                let x = tl_proj.x as i64 + n;
                let y = tl_proj.y as i64 + m;
                if 0 <= y && y < length_tile {
                    array.push((
                        zoom,
                        x,
                        y,
                        (x as f64 + 0.5 - center_proj.x).powf(2.)
                            + (y as f64 + 0.5 - center_proj.y).powf(2.),
                    ));
                }
            }
        }

        array.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

        dbg!(&array);
        Self {
            center: center_proj,
            zoom,
            array: array,
        }
    }
}
