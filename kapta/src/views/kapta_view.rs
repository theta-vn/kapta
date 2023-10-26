use crate::{
    consts::{BOUND_LAT_3857, BOUND_LON_3857},
    coords::{Coord, Proj, ProjCoord},
};

use super::SeriesPC;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KaptaView {
    pub center_p3857: ProjCoord,
    pub center: ProjCoord,
    pub top_left: ProjCoord,
    pub bottom_right: ProjCoord,
    pub origin: Coord,
    pub zoom: u8,
    pub width: u32,
    pub height: u32,
    pub pixel_x: f64,
    pub pixel_y: f64,
}

impl KaptaView {
    pub fn new(center_p3857: ProjCoord, origin: Coord, width: u32, height: u32, zoom: u8) -> Self {
        let (top_left, center, bottom_right) = center_p3857.bound_rec_tile(zoom, width, height);
        let length_hafl_tile = (2 as i64).pow((zoom - 1).into());
        let pixel_x = BOUND_LON_3857 / 256. / length_hafl_tile as f64;
        let pixel_y = BOUND_LAT_3857 / 256. / length_hafl_tile as f64;
        Self {
            center_p3857,
            center,
            origin,
            zoom,
            width,
            height,
            top_left,
            bottom_right,
            pixel_x,
            pixel_y,
        }
    }

    // TODO: doi center
    // pub fn change_center(&mut self, center: KProj)  {
    //     self.center_p3857 = center;
    // }

    pub fn new_collection(&self) -> SeriesPC {
        let mut collection: Vec<ProjCoord> = [].to_vec();
        let length_x = self.bottom_right.coord.x as i64 - self.top_left.coord.x as i64 + 1;
        let length_y = self.bottom_right.coord.y as i64 - self.top_left.coord.y as i64 + 1;

        let length_tile = (2 as i64).pow(self.zoom.into());

        for m in 0..length_y {
            for n in 0..length_x {
                let x = self.top_left.coord.x + n as f64;
                let y = self.top_left.coord.y + m as f64;
                if 0. <= y && y < length_tile as f64 {
                    let distance2 = (x.floor() + 0.5 - self.center.coord.x).powf(2.)
                        + (y.floor() + 0.5 - self.center.coord.y).powf(2.);
                    collection.push(ProjCoord {
                        coord: Coord { x, y: y },
                        kind: Proj::Tile,
                        distance2: distance2,
                    })
                }
            }
        }

        collection.sort_by(|a, b| a.distance2.partial_cmp(&b.distance2).unwrap());

        SeriesPC { series: collection }
    }

    pub fn change_collection(
        &self,
        bottom_right: ProjCoord,
        center: ProjCoord,
        top_left: ProjCoord,
    ) -> SeriesPC {
        let mut collection: Vec<ProjCoord> = [].to_vec();
        let length_x = bottom_right.coord.x as i64 - top_left.coord.x as i64 + 1;
        let length_y = bottom_right.coord.y as i64 - top_left.coord.y as i64 + 1;

        let length_tile = (2 as i64).pow(self.zoom.into());

        for m in 0..length_y {
            for n in 0..length_x {
                let x = top_left.coord.x + n as f64;
                let y = top_left.coord.y + m as f64;
                if 0. <= y && y < length_tile as f64 {
                    let distance2 = (x.floor() + 0.5 - center.coord.x).powf(2.)
                        + (y.floor() + 0.5 - center.coord.y).powf(2.);
                    collection.push(ProjCoord {
                        coord: Coord { x, y: y },
                        kind: Proj::Tile,
                        distance2: distance2,
                    })
                }
            }
        }

        collection.sort_by(|a, b| a.distance2.partial_cmp(&b.distance2).unwrap());
        SeriesPC { series: collection }
    }

    pub fn drap_change_bound(
        &self,
        delta_pixel_x: f64,
        delta_pixel_y: f64,
    ) -> (bool, ProjCoord, ProjCoord, ProjCoord, ProjCoord) {
        let delta_proj_x = delta_pixel_x * self.pixel_x;
        let delta_proj_y = delta_pixel_y * self.pixel_y;
        let proj_3857_new = ProjCoord::new(
            self.center_p3857.coord.x - delta_proj_x,
            self.center_p3857.coord.y - delta_proj_y,
        );

        let (top_left, center, bottom_right) =
            proj_3857_new.bound_rec_tile(self.zoom, self.width, self.height);

        let check = (self.top_left.coord.x as i64) > (top_left.coord.x as i64)
            || (self.top_left.coord.y as i64) > (top_left.coord.y as i64)
            || (self.bottom_right.coord.x as i64) < (bottom_right.coord.x as i64)
            || (self.bottom_right.coord.y as i64) < (bottom_right.coord.y as i64);
        (check, top_left, center, bottom_right, proj_3857_new)
    }

    pub fn to_img(&self, data: SeriesPC) -> Vec<(String, String)> {
        let mut vec_img: Vec<(String, String)> = [].to_vec();
        for data in data.series {
            let trans_x = (self.width / 2 - 128) as f64
                + (data.coord.x.floor() + 0.5 - self.center.coord.x) * 256.
                + self.origin.x;
            let trans_y = (self.height / 2 - 128) as f64
                + (data.coord.y.floor() + 0.5 - self.center.coord.y) * 256.
                + self.origin.y;
            let trans = format!("translate3d({}px, {}px, 0px)", trans_x, trans_y);
            let count = (2 as i64).pow(self.zoom.into());
            let url = format!(
                "https://tile.openstreetmap.org/{}/{}/{}.png",
                self.zoom,
                (data.coord.x as i64).rem_euclid(count),
                (data.coord.y as i64 % count),
            );
            vec_img.push((url, trans))
        }

        vec_img
    }
}
