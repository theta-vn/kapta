use geo_types::Coord;
use proj::Transform;
use std::fmt;
// use proj::{Proj, Coord};
// pub const BOUND_LON_3857: f64 = 20037508.34;
// pub const BOUND_LAT_3857: f64 = 20048966.1;
// pub const PI: f64 = std::f64::consts::PI;
// pub const E: f64 = std::f64::consts::E;

//  Projected bounds:
// -20037508.34 -20048966.1
// 20037508.34 20048966.1
// WGS84 bounds:
// -180.0 -85.06
// 180.0 85.06
#[derive(Debug, Default)]
pub enum CRS {
    #[default]
    EPSG4326, // WGS 84 | World
    EPSG3857, // WGS 84 / Pseudo-Mercato | World between 85.06°S and 85.06°N.
    EPSG4756, // VN-2000 | Vietnam-onshore | 1m
    Error,
}
impl fmt::Display for CRS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CRS::EPSG4326 => write!(f, "EPSG:4326"),
            CRS::EPSG3857 => write!(f, "EPSG:3857"),
            CRS::EPSG4756 => write!(f, "EPSG:4756"),
            _ => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Default)]
pub struct KCoord {
    pub coord: Coord,
    pub kind: CRS,
}

impl KCoord {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            coord: Coord { x, y },
            kind: CRS::default(),
        }
    }

    pub fn transformed_crs_to_crs(&self, crs: CRS) -> Self {
        let coord = self.coord;
        match coord.transformed_crs_to_crs(&self.kind.to_string(), &crs.to_string()) {
            Ok(coord) => Self {
                coord: coord,
                kind: crs,
            },
            Err(_) => Self {
                coord: self.coord,
                kind: CRS::Error,
            },
        }
    }

    // pub fn convert(&self, crs: CRS) -> Self {
    //     // let lon = (self.lon / 180_f64) * BOUND_LON_3857;
    //     // let lat_degrees = ((self.lat + 90_f64) * PI / 360_f64).tan().log(E) / (PI/90_f64);
    //     // let lat = (lat_degrees / 90_f64) * BOUND_LON_3857;

    //     let from = self.kind.to_string();
    //     let to = crs.to_string();
    //     let proj = Proj::new_known_crs(&from, &to, None).unwrap();
    //     dbg!(&proj);

    //     let (lon, lat) = proj.convert((107., 17.)).unwrap();

    //     Self {
    //         lon,
    //         lat,
    //         kind: CRS::EPSG3857,
    //     }
    // }

    // public double[] ConvertCoodinates()
    // {
    //     double[] xy = new double[2];
    //     xy[0] = 5085240.8300000000;
    //     xy[1] = 1530088.9600000000;
    // //An array for the z coordinate
    //     double[] z = new double[1];
    //     z[0] = 0;
    //     ProjectionInfo pStart = KnownCoordinateSystems.Geographic.World.WGS1984;
    //     pStart.AuthorityCode = 3857;
    //     ProjectionInfo pEnd = KnownCoordinateSystems.Geographic.World.WGS1984;
    //     pEnd.AuthorityCode = 4326;
    //     Reproject.ReprojectPoints(xy, z, pStart, pEnd, 0, 1);
    //     return xy;
    // }
}

// impl Coord<f64> for KCoord  {
//     fn x(&self) -> f64 {
//         self.lon
//     }
//     fn y(&self) -> f64 {
//         self.lat
//     }
//     fn from_xy(x: f64, y: f64) -> Self {
//         Self { lon: x, lat: y, kind: CRS::default() }
//     }
// }
