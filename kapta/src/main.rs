use geo_types::Coord;
use kapta::view;
use leptos::{ev::online, html::Div, *};
use serde::{Deserialize, Serialize};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KaptaState {
    pub center: Coord,
    pub zoom: u8,
    pub array: Vec<(u8, i64, i64, f64)>,
}

#[component]
pub fn App() -> impl IntoView {
    let (loaded, setLoaded) = create_signal(false);
    let state = create_rw_signal(KaptaState::default());
    let (center, setCenter) = create_slice(
        state,
        |state| state.center.clone(),
        |state, center| {
            state.center = center;
        },
    );

    let (zoom, setZoom) = create_slice(
        state,
        |state| state.zoom.clone(),
        |state, zoom| {
            state.zoom = zoom;
        },
    );

    let (array, setArray) = create_slice(
        state,
        |state| state.array.clone(),
        |state, array| {
            state.array = array;
        },
    );
    // log::debug!("{:#?}", state);
    let h: u32 = 700;
    let w: u32 = 900;
    let z: u8 = 1;
    let ct: Coord = (106.645, 10.788).into();

    let node_ref = create_node_ref::<Div>();

    setZoom.set(z);

    create_effect(move |_| {
        log::debug!("EFFECT========================={}", zoom.get());
        let view = view::render(w, h, zoom.get(), ct);
        setCenter.set(view.center);
        setArray.set(view.array);
    });

    view! {
        <div
            class="mx-auto relative overflow-hidden bg-primary-80"
            style:height=move || format!("{}px", h)
            style:width=move || format!("{}px", w)
        >
            <div id="control"
                class="top-0 left-0 z-50 absolute"
            >
                <button
                    class="bg-primary-80 block m-2 w-8 h-8"
                    on:click=move |_| {
                        log::debug!("CLICK {}", zoom.get());
                        setZoom.set(zoom.get() + 1);

                    }
                >
                    "+" {move || zoom.get()}
                </button>
                <button
                    class="bg-primary-80 block m-2 w-8 h-8"
                    on:click=move |_| {
                        log::debug!("CLICK {}", zoom.get());                        
                        setZoom.set(zoom.get() - 1);
                    }
                >
                    "-"{move || zoom.get()}
                </button>
            </div>
            <div
                id="base"
                ref=node_ref
                class="top-0 left-0 z-0"
                style="transform: translate3d(0px, 0px, 0px); opacity: 1;"
            >
                {move || array.get().iter().map(|data| {
                        let trans_x = (w / 2 - 128) as f64
                            + (data.1 as f64 + 0.5 - center.get().x) * 255.;
                        let trans_y = (h / 2 - 128) as f64
                            + (data.2 as f64 + 0.5 - center.get().y) * 255.;
                        let trans = format!("translate3d({}px, {}px, 0px)", trans_x, trans_y);
                        let count = (2 as i64).pow(zoom.get().into());
                        let url = format!(
                            "https://tile.openstreetmap.org/{}/{}/{}.png",
                            data.0,
                            (data.1 as i64).rem_euclid(count),
                            (data.2 as i64 % count),
                        );
                        view! {
                            <img
                                alt=""
                                src=url
                                class="absolute top-0 left-0"
                                // transform: translate3d(402px, 213px, 0px); opacity: 1;"
                                style="width: 256px; height: 256px; opacity: 1;"
                                style:transform=trans.clone()
                            />
                        }
                    })
                    .collect::<Vec<_>>()}

                <div
                    class="absolute"
                    style:top="350px"
                    style:left="450px"
                    style:width="2px"
                    style:height="2px"
                    style:background="red"
                ></div>
            </div>
        // </div>
        </div>
    }
    // view! {
    //     "cai gi day"
    // }
}
