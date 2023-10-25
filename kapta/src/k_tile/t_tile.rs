use geo_types::Coord;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tile {
    pub url: String,
    pub transform: String,
}

impl Tile {
    pub fn new(url: &str, transform: &str) -> Self {
        Self {
            url: url.to_owned(),
            transform: transform.to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TileCollect {
    pub collection: Vec<Tile>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KTile {
    pub x: i64,
    pub y: i64,
    pub z: u8,
}

impl KTile {
    // pub fn to_img(&self) -> (String, String) {
    //     let trans_x = (width / 2 - 128) as f64
    //             + (x as f64 + 0.5 - self.center.x) * 255. + topleft_x;
    //         let trans_y = (heigth / 2 - 128) as f64
    //             + (data.2 as f64 + 0.5 - self.center.y) * 255. + topleft_y;
    //         let trans = format!("translate3d({}px, {}px, 0px)", trans_x, trans_y);
    //         let count = (2 as i64).pow(self.zoom.into());
    //         let url = format!(
    //             "https://tile.openstreetmap.org/{}/{}/{}.png",
    //             data.0,
    //             (data.1 as i64).rem_euclid(count),
    //             (data.2 as i64 % count),
    //         );
    //         (url, trans)
    // }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KTileCollect {
    pub collection: Vec<KTile>,
    pub width: u32,
    pub heigth: u32,
    pub top_left: Coord,
    pub center: Coord
}
impl KTileCollect {
    // pub fn to_img(&self) -> (String, String) {

    //     let trans_x =
    //         (self.width / 2 - 128) as f64 + (x as f64 + 0.5 - self.center.x) * 255. + self.top_left.x;
    //     let trans_y =
    //         (heigth / 2 - 128) as f64 + (data.2 as f64 + 0.5 - self.center.y) * 255. + topleft_y;
    //     let trans = format!("translate3d({}px, {}px, 0px)", trans_x, trans_y);
    //     let count = (2 as i64).pow(self.zoom.into());
    //     let url = format!(
    //         "https://tile.openstreetmap.org/{}/{}/{}.png",
    //         data.0,
    //         (data.1 as i64).rem_euclid(count),
    //         (data.2 as i64 % count),
    //     );
    //     (url, trans)
    // }
}
