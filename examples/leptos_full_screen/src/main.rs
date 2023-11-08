use geojson::FeatureCollection;
use leptos::*;
use leptos_kapta::{Kapta, KaptaCoord};
use std::str::FromStr;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}
// ,
#[component]
pub fn App() -> impl IntoView {
    let geojson_str = r#"
    {
      "type": "FeatureCollection",
      "features": [        
        {
          "type": "Feature",
          "properties": {
            "address": "49 Pham Ngoc Thach",
            "name": "NHG",
            "kapta": {
              "draw": "marker",
              "tooltip": "NHG - 49 Pham Ngoc Thach"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.693125,  10.784967]
          }
        },        
        {
          "type": "Feature",
          "properties": {
            "address": "215 Dien Bien Phu",
            "name": "HIU",
            "kapta": {
              "draw": "marker",
              "tooltip": "HIU - 215 Dien Bien Phu"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.706443, 10.800130]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "215 Dien Bien Phu",
            "name": "HIU - Dam Sen",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.6334625, 10.7697885]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "8 Nguyen Van Trang",
            "name": "HSU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.6923016, 10.7703372]
          }
        }, 
        {
          "type": "Feature",
          "properties": {
            "address": "93 Cao Thang",
            "name": "HSU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.6790082, 10.7724525]
          }
        },        
        {
          "type": "Feature",
          "properties": {
            "address": "No 5 Street",
            "name": "HSU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.6301814, 10.8563364]
          }
        }, 
        {
          "type": "Feature",
          "properties": {
            "address": "7/1 Thanh Thai",
            "name": "HSU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.6650247, 10.7695139]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "187 Hoang Van Thu",
            "name": "GDU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.6713598, 10.7995006]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "371 Nguyen Kiem",
            "name": "GDU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.678565, 10.816635]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "80 Truong Cong Dinh",
            "name": "BVU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [107.0796154, 10.3485815]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "01 Truong Van Bang",
            "name": "BVU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [107.0833544, 10.3640894]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "951 Binh Gia",
            "name": "BVU",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [107.1164112, 10.3928078]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "QL 1A",
            "name": "MIT",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [107.1608595, 10.942756]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "No 5 Street",
            "name": "HSC",
            "kapta": {
              "draw": "marker"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.6297867,10.8566288]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "397 Ba Muoi Thang Tu",
            "name": "SNA",
            "kapta": {
              "draw": "marker",
              "color": "blue"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.8190479, 10.9469229]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "No 20 Street",
            "name": "SNA",
            "kapta": {
              "draw": "marker",
              "color": "blue"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.687580 , 10.730299]
          }
        },


        {
          "type": "Feature",
          "properties": {
            "address": "Ha Long",
            "name": "UKA",
            "kapta": {
              "draw": "marker",
              "color": "blue"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [107.100578, 20.951144]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "Hue",
            "name": "UKA",
            "kapta": {
              "draw": "marker",
              "color": "blue"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [107.604765, 16.493058]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "Da Nang",
            "name": "UKA",
            "kapta": {
              "draw": "marker",
              "color": "blue",
              "tooltip": "UKA - Da nang"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [108.2022375, 16.0728744]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "Quang Ngai",
            "name": "UKA",
            "kapta": {
              "draw": "marker",
              "color": "blue"              
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [108.8012167, 15.1015697]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "Binh Thanh",
            "name": "UKA",
            "kapta": {
              "draw": "marker",
              "color": "blue"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [106.7178262, 10.804729]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "Ba Ria",
            "name": "UKA",
            "kapta": {
              "draw": "marker",
              "color": "blue"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [107.175132, 10.499437]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "address": "Gia Lai",
            "name": "UKA",
            "kapta": {
              "draw": "marker",
              "color": "blue"
            }
          },
          "geometry": {
              "type": "Point",
              "coordinates": [108.0182292, 13.9583405]
          }
        },            
        {
          "type": "Feature",
          "properties": {
            "name": "iSchool",
            "kapta": {
              "draw": "marker",
              "color": "blue",
              "tooltip": "iSchool"
            }
          },
          "geometry": {
            "type": "MultiPoint",
            "coordinates": [
              [107.2838916, 21.0002723],
              [105.2738162, 19.6715972],
              [107.1149539, 16.7865675],
              [108.8012167, 15.1015697],
              [109.1806235, 13.7944869],
              [109.1921009, 12.2519063],
              [108.9928467, 11.5759958],
              [106.490461, 10.638866],
              [105.7284493, 9.2801498],
              [106.343225, 9.942689],
              [105.9756964, 9.6079271],
              [105.098709,9.992117],
              [105.4319541, 10.3931329]
            ]
          }
        },
        {
          "type": "Feature",
          "properties": {
            "name": "SGA",
            "kapta": {
              "draw": "marker",
              "color": "green",
              "tooltip": "Saigon Academy"
            }
          },
          "geometry": {
            "type": "MultiPoint",
            "coordinates": [
              [106.6899266, 10.7931165],
              [106.6595896,  10.7970525],
              [106.6971741, 10.8164872],
              [106.7681866, 10.8534767]]
          }
        }
      ]
    }
    "#;

    let geo_feature = match FeatureCollection::from_str(geojson_str) {
        Ok(feature) => feature,
        Err(_e) => {            
            FeatureCollection {
                bbox: None,
                features: vec![],
                foreign_members: None,
            }
        }
    };

    let center: KaptaCoord = KaptaCoord::new(106.6931669, 16.5);
    let win = leptos::window();
    let wwidth = win.inner_width().unwrap().as_f64().unwrap();
    let wheight = win.inner_height().unwrap().as_f64().unwrap();

    view! {
        <div class="flex justify-center">
            <Kapta
                zoom=6
                width=wwidth as u32
                height=wheight as u32
                center=center
                preload=0
                feature_collection=Some(geo_feature)
            />
        </div>
    }
}

// let geojson_str = r#"
//     {
//       "type": "FeatureCollection",
//       "features": [
//         {
//           "type": "Feature",
//           "properties": {
//             "population": 200,
//             "show": "marker"
//           },
//           "geometry": {
//             "type": "MultiLineString",
//             "coordinates": [
//               [
//                 [103.0, 11.0],
//                 [106.0, 16.0]
//               ],
//               [
//                 [105.0, 16.0],
//                 [101.0, 23.0]
//               ]
//             ]
//           }
//         },
//         {
//           "type": "Feature",
//           "properties": {
//             "population": 200,
//             "show": "marker"
//           },
//           "geometry": {
//             "type": "LineString",
//             "coordinates": [
//               [108, 23],
//               [112, 15],
//               [110, 9]
//             ]
//           }
//         },
//         {
//           "type": "Feature",
//           "properties": {
//             "population": 200,
//             "show": "marker"
//           },
//           "geometry": {
//             "type": "Point",
//             "coordinates": [106.645, 10.788]
//           }
//         },
//         {
//           "type": "Feature",
//           "properties": {
//             "population": 200,
//             "show": "marker"
//           },
//           "geometry": {
//             "type": "MultiPoint",
//             "coordinates": [[106.645, 19.788], [106.645, 17.788]]
//           }
//         }
//       ]
//     }
//     "#;
