use geojson::{Feature, GeoJson, Geometry, JsonObject, JsonValue, Value};
use leptos::*;
use leptos_kapta::{Kapta, KaptaCoord};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    // let geojson_str = r#"
    // {
    //     "type": "Feature",
    //     "properties": {
    //         "name": "Firestone Grill"
    //     },
    //     "geometry": {
    //         "type": "Point",
    //         "coordinates": [-120.66029,35.2812]
    //     }
    // }
    // "#;

    // let geojsonFeature = {
    //     type: "Feature",
    //     properties: {
    //       name: "Coors Field",
    //       amenity: "Baseball Stadium",
    //       popupContent: "This is where the Rockies play!",
    //     },
    //     geometry: {
    //       type: "Point",
    //       coordinates: [107.99404, 15.75621],
    //     },
    //   };

    // let geojson = geojson_str.parse::<GeoJson>().unwrap();

    let geometry = Geometry::new(Value::Point(vec![-120.66029, 35.2812]));

    let mut properties = JsonObject::new();
    properties.insert(String::from("name"), JsonValue::from("Firestone Grill"));

    let geojson_new = GeoJson::Feature(Feature {
        bbox: None,
        geometry: Some(geometry),
        id: None,
        properties: Some(properties),
        foreign_members: None,
    });
    log::debug!("{:#?}",geojson_new);

    let center: KaptaCoord = KaptaCoord::new(106.645, 10.788);

    view! {
        <div class="mx-auto">
            <h1 class="text-center m-8 text-2xl ">Example with leptos</h1>
            <div class="flex justify-center">
                <Kapta zoom=3 width=900 height=700 center=center preload=1/>
            </div>
        </div>
    }
}
