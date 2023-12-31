use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};

use geojson::FeatureCollection;
use kapta::coords::geojson_to_kaptageo;
#[test]
fn render() {
    let geojson_str = r#"
    {
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "properties": {
                    "population": 200,
                    "kapta": "marker"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [-112.0372, 46.608058]
                }
            },
            {
                "geometry": {
                  "coordinates": [
                    [
                      [
                        108.05018,
                        21.55238
                      ],
                      [
                        107.04342,
                        21.811899
                      ],
                      [
                        106.567273,
                        22.218205
                      ],
                      [
                        106.725403,
                        22.794268
                      ],
                      [
                        105.811247,
                        22.976892
                      ],
                      [
                        105.329209,
                        23.352063
                      ],
                      [
                        104.476858,
                        22.81915
                      ],
                      [
                        103.504515,
                        22.703757
                      ],
                      [
                        102.706992,
                        22.708795
                      ],
                      [
                        102.170436,
                        22.464753
                      ],
                      [
                        102.754896,
                        21.675137
                      ],
                      [
                        103.203861,
                        20.766562
                      ],
                      [
                        104.435,
                        20.758733
                      ],
                      [
                        104.822574,
                        19.886642
                      ],
                      [
                        104.183388,
                        19.624668
                      ],
                      [
                        103.896532,
                        19.265181
                      ],
                      [
                        105.094598,
                        18.666975
                      ],
                      [
                        105.925762,
                        17.485315
                      ],
                      [
                        106.556008,
                        16.604284
                      ],
                      [
                        107.312706,
                        15.908538
                      ],
                      [
                        107.564525,
                        15.202173
                      ],
                      [
                        107.382727,
                        14.202441
                      ],
                      [
                        107.614548,
                        13.535531
                      ],
                      [
                        107.491403,
                        12.337206
                      ],
                      [
                        105.810524,
                        11.567615
                      ],
                      [
                        106.24967,
                        10.961812
                      ],
                      [
                        105.199915,
                        10.88931
                      ],
                      [
                        104.334335,
                        10.486544
                      ],
                      [
                        105.076202,
                        9.918491
                      ],
                      [
                        104.795185,
                        9.241038
                      ],
                      [
                        105.158264,
                        8.59976
                      ],
                      [
                        106.405113,
                        9.53084
                      ],
                      [
                        107.220929,
                        10.364484
                      ],
                      [
                        108.36613,
                        11.008321
                      ],
                      [
                        109.200136,
                        11.666859
                      ],
                      [
                        109.33527,
                        13.426028
                      ],
                      [
                        108.877107,
                        15.276691
                      ],
                      [
                        108.269495,
                        16.079742
                      ],
                      [
                        107.361954,
                        16.697457
                      ],
                      [
                        106.426817,
                        18.004121
                      ],
                      [
                        105.662006,
                        19.058165
                      ],
                      [
                        105.881682,
                        19.75205
                      ],
                      [
                        106.715068,
                        20.696851
                      ],
                      [
                        108.05018,
                        21.55238
                      ]
                    ]
                  ],
                  "type": "Polygon"
                },
                "id": "VNM",
                "properties": {
                  "name": "Vietnam"
                },
                "type": "Feature"
            }
        ]
    }
    "#;

    let geo_feature = FeatureCollection::from_str(geojson_str).unwrap();

    let array = geojson_to_kaptageo(geo_feature);
    dbg!(&array);
    for m in array {
        let mut hasher = DefaultHasher::new();
        m.hash(&mut hasher);
        dbg!(hasher.finish());
        // 1540136902312859152
        // 14667932546091814007
    }
}

#[test]
fn convert() {
    let geojson_str = r#"
  {
      "type": "FeatureCollection",
      "features": [
          {
              "type": "Feature",
              "properties": {
                "address": "49 Pham Ngoc Thach",
                "kapta": {
                  "show": "marker"
                }
              },
              "geometry": {
                  "type": "Point",
                  "coordinates": [106.690438, 10.7849054]
              }
          }
      ]
  }
  "#;

    let geo_feature = FeatureCollection::from_str(geojson_str).unwrap();
    dbg!(&geo_feature);

    let array = geojson_to_kaptageo(geo_feature);
    dbg!(&array);
    for m in array {
        let mut hasher = DefaultHasher::new();
        m.hash(&mut hasher);
        dbg!(hasher.finish());
        // 1540136902312859152
        // 14667932546091814007
    }
}
