use std::hash::{Hash, Hasher};

use crate::coords::KaptaCoord;
use geojson::{FeatureCollection, JsonObject};

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum KaptaGeo {
    Point(KaptaPoint),
    MultiPoint(Vec<KaptaPoint>),
    LineString(KaptaLineString),
    MultiLineString(Vec<KaptaLineString>),
    Polygon(KaptaPolygon),
    MultiPolygon(Vec<KaptaPolygon>),
    // GeometryCollection(Vec<Geometry>),
}
impl Eq for KaptaGeo {}

pub type SeriesKG = Vec<KaptaGeo>;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KaptaPoint {
    pub value: [f64; 2],
    pub properties: Option<JsonObject>,
}

impl Hash for KaptaPoint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{:.6}", self.value[0]).hash(state);
        format!("{:.6}", self.value[1]).hash(state);
    }
}

impl KaptaPoint {
    pub fn new(value: [f64; 2], properties: Option<JsonObject>) -> Self {
        Self { value, properties }
    }
}

impl From<[f64; 2]> for KaptaPoint {
    fn from(value: [f64; 2]) -> Self {
        Self {
            value,
            properties: None,
        }
    }
}

impl Eq for KaptaPoint {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KaptaPolygon {
    pub value: Vec<Vec<[f64; 2]>>,
    pub properties: Option<JsonObject>,
}
impl Hash for KaptaPolygon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let flat: Vec<[f64; 2]> = self.value.clone().into_iter().flatten().collect();
        for m in flat {
            format!("{:.6}", m[0]).hash(state);
            format!("{:.6}", m[1]).hash(state);
        }
    }
}

impl KaptaPolygon {
    pub fn new(value: Vec<Vec<[f64; 2]>>, properties: Option<JsonObject>) -> Self {
        Self { value, properties }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KaptaLineString {
    pub value: Vec<[f64; 2]>,
    pub properties: Option<JsonObject>,
}
impl Hash for KaptaLineString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for m in self.value.clone() {
            format!("{:.6}", m[0]).hash(state);
            format!("{:.6}", m[1]).hash(state);
        }
    }
}

impl KaptaLineString {
    pub fn new(value: Vec<[f64; 2]>, properties: Option<JsonObject>) -> Self {
        Self { value, properties }
    }
}

pub fn geojson_to_kaptageo(geo_feature: FeatureCollection) -> Vec<KaptaGeo> {
    let mut array: Vec<KaptaGeo> = [].to_vec();
    for (_pos, geo_jf) in geo_feature.features.iter().enumerate() {
        let geo = geo_jf.clone();
        let geo_value = geo.geometry.unwrap().value;
        let geo_prop = geo.properties;

        match geo_value {
            geojson::Value::Point(point) => {                
                let coord = KaptaCoord::new(point[0], point[1]);
                let proj_coord = coord.to_proj_coord();
                let kapta_point = KaptaPoint::new([proj_coord.x, proj_coord.y], geo_prop);
                array.push(KaptaGeo::Point(kapta_point));
            }
            geojson::Value::MultiPoint(multi_point) => {
                let mut multi_value: Vec<KaptaPoint> = [].to_vec();
                for point in multi_point {
                    let coord = KaptaCoord::new(point[0], point[1]);
                    let proj_coord = coord.to_proj_coord();
                    let kapta_point = KaptaPoint::new([proj_coord.x, proj_coord.y], geo_prop.clone());
                    multi_value.push(kapta_point);
                }
                array.push(KaptaGeo::MultiPoint(multi_value));
            }
            geojson::Value::LineString(line_string) => {
                let mut value: Vec<[f64; 2]> = [].to_vec();
                for point in line_string {
                    let coord = KaptaCoord::new(point[0], point[1]).to_proj_coord();
                    value.push([coord.x, coord.y]);
                }
                array.push(KaptaGeo::LineString(KaptaLineString::new(value, geo_prop)))
            }
            geojson::Value::MultiLineString(multi_line_string) => {
                let mut multi_value: Vec<KaptaLineString> = [].to_vec();
                for line_string in multi_line_string {
                    let mut value_line: Vec<[f64; 2]> = [].to_vec();
                    for point in line_string {
                        let coord = KaptaCoord::new(point[0], point[1]).to_proj_coord();
                        value_line.push([coord.x, coord.y]);
                    }
                    multi_value.push(KaptaLineString::new(value_line, geo_prop.clone()));
                }

                array.push(KaptaGeo::MultiLineString(multi_value));
            }
            geojson::Value::Polygon(polygon) => {
                let mut value: Vec<Vec<[f64; 2]>> = [].to_vec();
                for poly in polygon {
                    let mut value_poly: Vec<[f64; 2]> = [].to_vec();
                    for point in poly {
                        let coord = KaptaCoord::new(point[0], point[1]).to_proj_coord();
                        value_poly.push([coord.x, coord.y]);
                    }
                    value.push(value_poly);
                }
                array.push(KaptaGeo::Polygon(KaptaPolygon::new(value, geo_prop)))
            }
            geojson::Value::MultiPolygon(multi_polygon) => {
                let mut multi_value: Vec<KaptaPolygon> = [].to_vec();
                for polygon in multi_polygon {
                    let mut value_polygon: Vec<Vec<[f64; 2]>> = [].to_vec();
                    for poly in polygon {
                        let mut value_poly: Vec<[f64; 2]> = [].to_vec();
                        for point in poly {
                            let coord = KaptaCoord::new(point[0], point[1]).to_proj_coord();
                            value_poly.push([coord.x, coord.y]);
                        }
                        value_polygon.push(value_poly);
                    }
                    multi_value.push(KaptaPolygon::new(value_polygon, geo_prop.clone()));
                }
                array.push(KaptaGeo::MultiPolygon(multi_value));
            }
            geojson::Value::GeometryCollection(_) => {}
        }
    }
    array
}
