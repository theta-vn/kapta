use std::{ops::Div, usize};

use geo_types::Coord;

use crate::{
    consts::*,
    k_geo::{KCoord, CRS},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TView {
    pub cx: u32,
    pub cy: u32,

    pub zoom: u8,
    pub xt: u32,
    pub yt: u32,
    pub array: Vec<(u8, i64, i64, i64)>,
}

impl TView {
    pub fn tile_size(zoom: u8) -> (f64, f64) {
        let length_tile = (2 as u64).pow(zoom.into());
        let tile_width = (BOUND_LON_3857 * 2.).div(length_tile as f64);
        let tile_heigth = (BOUND_LAT_3857 * 2.).div(length_tile as f64);
        (tile_width, tile_heigth)
    }

    pub fn load(center: Coord, zoom: u8, width: u32, heigth: u32) -> Self {
        let center_kc_4326 = KCoord::from(center);
        let center_kc_3857 = center_kc_4326.transformed(CRS::EPSG3857);
        let center = center_kc_3857.to_tile_coord(zoom);
        let (xt, yt) = center_kc_3857.translate3d(zoom);

        let (tile_width, tile_heigth) = Self::tile_size(zoom);
        let dx = ((width / 2 - 1) as f64 / TILE as f64) * tile_width;
        let dy = ((heigth / 2 - 1) as f64 / TILE as f64) * tile_heigth;
        dbg!(dx, dy);
        let tl_kc_3876 = center_kc_3857.translate(-dx, dy);
        let br_kc_3876 = center_kc_3857.translate(dx, -dy);
        dbg!(center_kc_3857, tl_kc_3876, br_kc_3876);
        let tl_coord = tl_kc_3876.to_tile_coord(zoom);
        let br_coord = br_kc_3876.to_tile_coord(zoom);
        dbg!(center, tl_coord, br_coord);
        let length_x = br_coord.x - tl_coord.x + 1.;
        let length_y = br_coord.y - tl_coord.y + 1.;
        dbg!(length_x, length_y);
        let mut array: Vec<(u8, i64, i64, i64)> = [].to_vec();
        // let mut array_string: Vec<String> = [].to_vec();
        let length_tile = (2 as u64).pow(zoom.into());

        for m in 0..length_y as i64 {
            for n in 0..length_x as i64 {
                // let trans = format!("translate3d({}px, {}px, 0px)", xt, yt);
                // let url = format!(
                //     "https://tile.openstreetmap.org/{}/{}/{}.png",
                //     zoom,
                //     ((tl_coord.x as usize + n) as u64) % length_tile,
                //     ((tl_coord.y as usize + m) as u64) % length_tile
                // );
                // array_string.push(url);
                let x = tl_coord.x as i64 + n;
                let y = tl_coord.y as i64 + m;
                array.push((
                    zoom,
                    x,
                    y,
                    (x - center.x as i64).pow(2) + (y - center.y as i64).pow(2),
                ));
            }
        }
        // dbg!(&array);
        array.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());
        // order(center.x as usize, center.y as usize, &array);
        dbg!(&array);
        Self {
            cx: center.x as u32,
            cy: center.y as u32,
            zoom,
            xt,
            yt,
            array: array,
        }
    }
}

// pub fn order(x: usize, y: usize, slice: &[(u8, usize,  usize)]) {
//     dbg!(x, y);
//     dbg!(slice);
//     let mut dist: Vec<(usize, i64)> = slice.iter().enumerate().map(|(post, v)| {
//         (post, ((v.1 as i64 - x as i64).pow(2) + (v.2 as i64 - y as i64).pow(2)))
//     }).collect::<Vec<_>>();
//     dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
//     dbg!(dist);
//     // slice.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     // dbg!(slice);
// }
