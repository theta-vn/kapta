use geo_types::Coord;
use geojson::JsonObject;

#[derive(Clone, Debug, PartialEq)]
pub enum KaptaGeo {
    Point(KaptaPoint),
    // MultiPoint(Vec<PointType>),
    // LineString(LineStringType),
    // MultiLineString(Vec<LineStringType>),
    Polygon(KaptaPolygon),
    // MultiPolygon(Vec<PolygonType>),
    // GeometryCollection(Vec<Geometry>),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KaptaPoint {
    pub value: [f64; 2],
    pub properties: Option<JsonObject>,
}

impl KaptaPoint {
    pub fn new(value: [f64; 2], properties: Option<JsonObject>) -> Self {
        Self { value, properties }
    }
}



#[derive(Debug, Default, Clone, PartialEq)]
pub struct KaptaPolygon {
    pub value: Vec<Vec<Vec<f64>>>,
    pub properties: Option<JsonObject>,
}

impl KaptaPolygon {
    pub fn new(value: Vec<Vec<Vec<f64>>>, properties: Option<JsonObject>) -> Self {
        Self { value, properties }
    }
}
