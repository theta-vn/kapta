use std::{
    fmt::format,
    hash::{Hash, Hasher},
};

use crate::coords::KaptaCoord;
use geojson::{FeatureCollection, JsonObject};

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum KaptaGeo {
    Point(KaptaPoint),
    // MultiPoint(Vec<PointType>),
    // LineString(LineStringType),
    // MultiLineString(Vec<LineStringType>),
    Polygon(KaptaPolygon),
    // MultiPolygon(Vec<PolygonType>),
    // GeometryCollection(Vec<Geometry>),
}
impl Eq for KaptaGeo {}

// #[derive(Clone, Debug, PartialEq, Default)]
// pub struct SeriesKG {
//     pub series: Vec<KaptaGeo>
// }

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

pub fn geojson_to_kaptageo(geo_feature: FeatureCollection) -> Vec<KaptaGeo> {
    let mut array: Vec<KaptaGeo> = [].to_vec();
    for (pos, geo_jf) in geo_feature.features.iter().enumerate() {
        dbg!(pos);
        dbg!(geo_jf);
        let geo = geo_jf.clone();
        let geo_value = geo.geometry.unwrap().value;
        dbg!(&geo_value);
        let geo_prop = geo.properties;

        match geo_value {
            geojson::Value::Point(point) => {
                let coord = KaptaCoord::new(point[0], point[1]).to_proj_coord();

                let kapta_point = KaptaPoint::new([coord.x, coord.y], geo_prop);
                array.push(KaptaGeo::Point(kapta_point));
            }
            geojson::Value::MultiPoint(_) => todo!(),
            geojson::Value::LineString(_) => todo!(),
            geojson::Value::MultiLineString(_) => todo!(),
            geojson::Value::Polygon(polygon) => {
                // dbg!(polygon)
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
            geojson::Value::MultiPolygon(_) => todo!(),
            geojson::Value::GeometryCollection(_) => todo!(),
        }
    }
    array
}
