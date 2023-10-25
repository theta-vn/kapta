use super::{KCoord, CRS};
use crate::consts::{BOUND_LAT_3857, BOUND_LON_3857, TILE};
use geo_types::Coord;
use std::ops::Div;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Proj {
    #[default]
    EPSG3857, // WGS 84 / Pseudo-Mercato | World between 85.06°S and 85.06°N.
    Tile,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct KProj {
    pub coord: Coord,
    pub kind: Proj,
    pub distance2: f64,
}

impl From<KCoord> for KProj {
    fn from(kcoord: KCoord) -> Self {
        let c = kcoord.transformed(CRS::EPSG3857);
        let x = c.coord.x + BOUND_LON_3857;
        let y = BOUND_LAT_3857 - c.coord.y;
        Self {
            coord: Coord { x, y },
            kind: Proj::default(),
            distance2: 0.,
        }
    }
}

impl KProj {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            coord: Coord { x, y },
            kind: Proj::EPSG3857,
            distance2: 0.,
        }
    }

    pub fn similar(&self, other: KProj) -> bool {
        self.coord.x as i64 == other.coord.x as i64 && self.coord.y as i64 == other.coord.y as i64
    }


    pub fn to_tile(&self, zoom: u8) -> Self {
        match self.kind {
            Proj::EPSG3857 => {
                let (tile_width, tile_heigth) = size_tile(zoom);

                let cx_tile = self.coord.x / tile_width;
                let cy_tile = self.coord.y / tile_heigth;

                Self {
                    coord: Coord {
                        x: cx_tile,
                        y: cy_tile,
                    },
                    kind: Proj::Tile,
                    distance2: 0.,
                }
            }
            Proj::Tile => *self,
        }
    }

    pub fn bound_rec_tile(&self, zoom: u8, width: u32, heigth: u32) -> (KProj, KProj, KProj) {
        let center_tile = self.to_tile(zoom);
        dbg!(center_tile);
        let dx = (width / 2 - 1) as f64 / TILE as f64;
        let dy = (heigth / 2 - 1) as f64 / TILE as f64;

        let tl_tile = Coord {
            x: center_tile.coord.x - dx,
            y: center_tile.coord.y - dy,
        };
        let br_tile = Coord {
            x: center_tile.coord.x + dx,
            y: center_tile.coord.y + dy,
        };
        (
            KProj {
                coord: tl_tile,
                kind: Proj::Tile,
                distance2: 0.,
            },
            center_tile,
            KProj {
                coord: br_tile,
                kind: Proj::Tile,
                distance2: 0.,
            },
        )
    }
}

pub fn size_tile(zoom: u8) -> (f64, f64) {
    let length_tile = (2 as u64).pow(zoom.into());
    let tile_width = (BOUND_LON_3857 * 2.).div(length_tile as f64);
    let tile_heigth = (BOUND_LAT_3857 * 2.).div(length_tile as f64);
    (tile_width, tile_heigth)
}
