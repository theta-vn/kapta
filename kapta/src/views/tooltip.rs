use geo_types::Coord;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tooltip {
    pub coor: Coord,
    pub html: String,
}
