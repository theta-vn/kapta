use geo_types::Coord;
// use proj::Transform;
use std::{fmt, ops::Div};

use crate::{consts::TILE, k_tile::TView};
pub const BOUND_LON_3857: f64 = 20_048_966.1;
pub const BOUND_LAT_3857: f64 = 20_037_508.34;
pub const PI: f64 = std::f64::consts::PI;
pub const E: f64 = std::f64::consts::E;

//  Projected bounds:
// -20037508.34 -20048966.1
// 20037508.34 20048966.1
// WGS84 bounds:
// -180.0 -85.06
// 180.0 85.06
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CRS {
    #[default]
    EPSG4326, // WGS 84 | World
    EPSG3857, // WGS 84 / Pseudo-Mercato | World between 85.06°S and 85.06°N.
    // EPSG4756, // VN-2000 | Vietnam-onshore | 1m
}
impl fmt::Display for CRS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CRS::EPSG4326 => write!(f, "EPSG:4326"),
            CRS::EPSG3857 => write!(f, "EPSG:3857"),
            // CRS::EPSG4756 => write!(f, "EPSG:4756"),
            _ => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct KCoord {
    pub coord: Coord,
    pub kind: CRS,
}

impl From<Coord> for KCoord {
    fn from(coord: Coord) -> Self {
        Self { coord, kind: CRS::default() }
    }
}

impl KCoord {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            coord: Coord { x, y },
            kind: CRS::default(),
        }
    }

	pub fn translate(&self, dx: f64, dy: f64) -> Self {
        Self {
            coord: Coord { x: self.coord.x + dx, y: self.coord.y + dy },
            kind: self.kind,
        }
    }
    pub fn translate3d(&self, zoom: u8) -> (u32, u32) {
        let (tile_width, tile_heigth) = TView::tile_size(zoom);        
        let cx_tile = self.coord.x / tile_width;
        let cy_tile = self.coord.y / tile_heigth;
        let x_translate = (cx_tile.fract() * TILE as f64) as u32;
        let y_translate = ((1.0 - cy_tile.fract()) * TILE as f64) as u32;
        (x_translate, y_translate)
    }

    pub fn to_tile_coord(&self, zoom: u8) -> Coord {
        let coord_3857 = self.transformed(CRS::EPSG3857);
        let length_tile = (2 as u64).pow(zoom.into());
        let (tile_width, tile_heigth) = TView::tile_size(zoom);
        
        let cx_tile = coord_3857.coord.x / tile_width;
        let cy_tile = coord_3857.coord.y / tile_heigth;
        let x =  cx_tile.ceil() + (length_tile / 2 - 1) as f64;
        let y = - cy_tile.ceil() + (length_tile / 2) as f64;
        let x_translate = (cx_tile.fract() * TILE as f64) as u32;
        let y_translate = ((1.0 - cy_tile.fract()) * TILE as f64) as u32;
        // (zoom, x as u32, y as u32, x_translate, y_translate)
        Coord { x, y }
    }


    // pub fn transformed_crs_to_crs(&self, crs: CRS) -> Self {
    //     let coord = self.coord;
    //     match coord.transformed_crs_to_crs(&self.kind.to_string(), &crs.to_string()) {
    //         Ok(coord) => Self {
    //             coord: coord,
    //             kind: crs,
    //         },
    //         Err(_) => Self {
    //             coord: self.coord,
    //             kind: CRS::Error,
    //         },
    //     }
    // }

    pub fn transformed(&self, crs: CRS) -> Self {
        match (&self.kind, crs) {
            (CRS::EPSG4326, CRS::EPSG4326) => *self,
            (CRS::EPSG4326, CRS::EPSG3857) => {
                let lon = (self.coord.x / 180_f64) * BOUND_LON_3857;
                let lat_degrees =
                    ((self.coord.y + 90_f64) * PI / 360_f64).tan().log(E) / (PI / 90_f64);
                let lat = (lat_degrees / 90_f64) * BOUND_LAT_3857;

                Self {
                    coord: Coord { x: lon, y: lat },
                    kind: crs,
                }
            }
            // (CRS::EPSG4326, CRS::EPSG4756) => todo!(),
            (CRS::EPSG3857, CRS::EPSG4326) => {
                let lon = (self.coord.x / BOUND_LON_3857) * 180_f64;
                let lat_degrees = (self.coord.y / BOUND_LAT_3857) * 90_f64;
                let lat = E.powf(lat_degrees * (PI / 90_f64)).atan() / (PI / 360_f64) - 90_f64;
                Self {
                    coord: Coord { x: lon, y: lat },
                    kind: crs,
                }
            }
            (CRS::EPSG3857, CRS::EPSG3857) => *self,
            // (CRS::EPSG3857, CRS::EPSG4756) => todo!(),
            // (CRS::EPSG4756, CRS::EPSG4326) => todo!(),
            // (CRS::EPSG4756, CRS::EPSG3857) => todo!(),
            // (CRS::EPSG4756, CRS::EPSG4756) => *self,
        }
    }
}

impl approx::AbsDiffEq for KCoord {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1.0e-6
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        &self.kind == &other.kind && Coord::abs_diff_eq(&self.coord, &other.coord, epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        &self.kind != &other.kind || !Coord::abs_diff_eq(&self.coord, &other.coord, epsilon)
    }
}

impl approx::RelativeEq for KCoord {
    fn default_max_relative() -> Self::Epsilon {
        1.0e-6
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        &self.kind == &other.kind
            && Coord::relative_eq(&self.coord, &other.coord, epsilon, max_relative)
    }
}
