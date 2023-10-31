use geojson::FeatureCollection;
use kapta::coords::{geojson_to_kaptageo, SeriesKG};
use leptos::{*, For};

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
pub fn GeoJsonLayer(
    zoom: ReadSignal<u8>,
    #[prop(default = None)] feature_collection: Option<FeatureCollection>,
) -> impl IntoView {
    let (data, set_data) = create_signal(SeriesKG::default());
    create_effect(move |_| {
        if let Some(collection) = &feature_collection {
            let kapta_geo = geojson_to_kaptageo(collection.clone());
            set_data.set(kapta_geo);
        }
    });
   
    view! {
        <div id="kapta-layer-gjson">
            <For
                each=move || data.get()
                key=|state| state.clone()
                let:data
            >
                <p>{log::debug!("{:#?}", data)}</p>
            </For>
        </div>
    }
    // match feature_collection {
    //     Some(collection) => {
    //         let kapta_geo = geojson_to_kaptageo(collection);
    //         log::debug!("{:#?}", &kapta_geo);
    //         // let geo =
    //         view! {
    //             <div id="kapta-layer-gjson">
    //                 // <For
    //                 //     each=kapta_geo
    //                 //     key=|state| state.clone()
    //                 //     let:child
    //                 // >
    //                 //     <p></p>
    //                 // </For>
    //             </div>
    //         }
    //     }
    //     None => view! {<div></div>},
    // }
}

#[component]
pub fn GeoPoint(
    zoom: ReadSignal<u8>,
    #[prop(default = None)] feature_collection: Option<FeatureCollection>,
) -> impl IntoView {
    match feature_collection {
        Some(collection) => {
            let kapta_geo = geojson_to_kaptageo(collection);
            log::debug!("{:#?}", kapta_geo);
            // let geo =
            view! {
                <div id="kapta-layer-gjson">
                    <svg xmlns="http://www.w3.org/2000/svg" width="24px" height="24px" viewBox="0 0 24 24" fill="none"
                        style="position: relative;top: 300px; left: 300px; z-index: 10"
                    >
                        <path fill="#ff0000" fill-rule="evenodd" d="M11.291 21.706 12 21l-.709.706zM12 21l.708.706a1 1 0 0 1-1.417 0l-.006-.007-.017-.017-.062-.063a47.708 47.708 0 0 1-1.04-1.106 49.562 49.562 0 0 1-2.456-2.908c-.892-1.15-1.804-2.45-2.497-3.734C4.535 12.612 4 11.248 4 10c0-4.539 3.592-8 8-8 4.408 0 8 3.461 8 8 0 1.248-.535 2.612-1.213 3.87-.693 1.286-1.604 2.585-2.497 3.735a49.583 49.583 0 0 1-3.496 4.014l-.062.063-.017.017-.006.006L12 21zm0-8a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" clip-rule="evenodd"/>
                    </svg>
                </div>
            }
        }
        None => view! {<div></div>},
    }
}
