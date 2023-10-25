use geo_types::Coord;
use crate::k_geo::{KCoord, KProj, Proj};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KCollection {
    pub collection: Vec<KProj>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KView {
    pub center: KProj,
    pub top_left: KProj,
    pub bottom_right: KProj,
    pub origin: Coord,
    pub zoom: u8,
    pub width: u32,
    pub height: u32,
    // pub data: KCollection,
}

impl KView {
    pub fn new(center: Coord, origin: Coord, width: u32, height: u32, zoom: u8) -> Self {
        let center_3857 = KCoord::from(center).transformed(crate::k_geo::CRS::EPSG3857);
        let kproj = KProj::from(center_3857).to_tile(zoom);        
        let (top_left, center, bottom_right) = kproj.bound_rec_tile(zoom, width, height);
        Self {
            center,
            origin,
            zoom,
            width,
            height,            
            top_left,
            bottom_right,
        }
    }

    pub fn new_collection(&self) -> KCollection {
        let mut collection: Vec<KProj> = [].to_vec();
        let length_x = self.bottom_right.coord.x as i64 - self.top_left.coord.x as i64 + 1;
        let length_y = self.bottom_right.coord.y as i64 - self.top_left.coord.y as i64 + 1;

        let length_tile = (2 as i64).pow(self.zoom.into());

        for m in 0..length_y {
            for n in 0..length_x {
                let x = self.top_left.coord.x + n as f64;
                let y = self.top_left.coord.y + m as f64;
                if 0. <= y && y < length_tile as f64 {
                    let distance2 = (x.floor() + 0.5 - self.center.coord.x).powf(2.)
                        + (y.floor()  + 0.5 - self.center.coord.y).powf(2.);
                    collection.push(KProj {
                        coord: Coord { x, y: y },
                        kind: Proj::Tile,
                        distance2: distance2,
                    })
                }
            }
        }

        collection.sort_by(|a, b| a.distance2.partial_cmp(&b.distance2).unwrap());
        // dbg!(&array);
        // let mut array: Vec<(u8, i64, i64, f64)> = [].to_vec();
        // Self {
        //     center: center_proj_tile,
        //     top_left: tl_proj_tile,
        //     bottom_right: br_proj_tile,
        //     zoom,
        //     array: array,
        // }

        KCollection { collection }
        // KCollection { collection: () }
    }

    pub fn to_img(&self, data: KCollection)-> Vec<(String, String)> {
        let mut vec_img: Vec<(String, String)> = [].to_vec();
        for data in data.collection {
            dbg!(self.width / 2 - 128,(data.coord.x as f64 + 0.5 - self.center.coord.x) * 256.);
            let trans_x = (self.width / 2 - 128) as f64
                + (data.coord.x.floor() + 0.5 - self.center.coord.x) * 256. + self.origin.x;
            let trans_y = (self.height / 2 - 128) as f64
                + (data.coord.y.floor() + 0.5 - self.center.coord.y) * 256. + self.origin.y;
            let trans = format!("translate3d({}px, {}px, 0px)", trans_x, trans_y);
            let count = (2 as i64).pow(self.zoom.into());
            let url = format!(
                "https://tile.openstreetmap.org/{}/{}/{}.png",
                self.zoom,
                (data.coord.x as i64).rem_euclid(count),
                (data.coord.y as i64 % count),
            );
            vec_img.push((url, trans))
        };
        
        
        vec_img
    }
}
