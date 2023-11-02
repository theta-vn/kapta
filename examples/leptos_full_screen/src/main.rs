use geojson::FeatureCollection;
use leptos::*;
use leptos_kapta::{Kapta, KaptaCoord};
use std::str::FromStr;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let geojson_str = r#"
    {
      "type": "FeatureCollection",
      "features": [       
        {
          "type": "Feature",
          "properties": {
            "population": 200,
            "show": "marker"
          },
          "geometry": {
            "type": "MultiLineString",
            "coordinates": [
              [
                [103.0, 11.0],
                [106.0, 16.0]
              ],
              [
                [105.0, 16.0],
                [101.0, 23.0]
              ]
            ]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "population": 200,
            "show": "marker"
          },
          "geometry": {
            "type": "LineString",
            "coordinates": [
              [108, 23],
              [112, 15],
              [110, 9]
            ]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "population": 200,
            "show": "marker"
          },
          "geometry": {
            "type": "Point",
            "coordinates": [106.645, 10.788]
          }
        }
      ]
    }
    "#;

    let geo_feature = match FeatureCollection::from_str(geojson_str) {
        Ok(feature) => feature,
        Err(_) => FeatureCollection {
            bbox: None,
            features: vec![],
            foreign_members: None,
        },
    };

    let center: KaptaCoord = KaptaCoord::new(106.645, 17.);
    let win = leptos::window();
    let wwidth = win.inner_width().unwrap().as_f64().unwrap();
    let wheight = win.inner_height().unwrap().as_f64().unwrap();
    
    view! {
        <div class="flex justify-center">
            <Kapta
                zoom=5
                width=wwidth as u32
                height=wheight as u32
                center=center
                preload=0
                feature_collection=Some(geo_feature)
            />
        </div>
    }
}
