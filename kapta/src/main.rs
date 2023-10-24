use geo_types::Coord;
use kapta::view;
use leptos::{html::Div, *};
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn, core::Position};


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let (center, setCenter) = create_signal(Coord { x: 0., y: 0. });
    let (zoom, setZoom) = create_signal(0);
    let slice: Vec<(u8, i64, i64, f64)> = [].to_vec();
    let (topleft, setTopleft) = create_signal(Coord { x: 0., y: 0. });

    let (array, setArray) = create_signal(slice);

    let h: u32 = 700;
    let w: u32 = 900;
    let z: u8 = 7;
    let ct: Coord = (106.645, 10.788).into();

    let div_ref = create_node_ref::<Div>();

    setZoom.set(z);

    create_effect(move |_| {
        let view = view::render(w, h, zoom.get(), ct);
        setCenter.set(view.center);
        setArray.set(view.array);
        let bound = div_ref.get().unwrap().get_bounding_client_rect();
        setTopleft.set(Coord {
            y: bound.top(),
            x: bound.left(),
        })
    });

    let UseDraggableReturn { x, y, .. } = use_draggable_with_options(
        div_ref,
        UseDraggableOptions::default().prevent_default(true)//.initial_value(Position {x: -84.5, y: 0. }),
    );

    view! {
        <div class="">
        <div
            class="mx-auto relative overflow-hidden bg-primary-80"
            style:height=move || format!("{}px", h)
            style:width=move || format!("{}px", w)
        >
            <div id="control"
                class="top-0 right-0 z-50 absolute"
            >
                <button
                    class="bg-primary-80 block w-8 h-6"
                    on:click=move |_| {
                        log::debug!("CLICK {}", zoom.get());
                        setZoom.set(zoom.get() + 1);

                    }
                >
                    "+"
                </button>
                <button
                    class="bg-primary-80 block w-8 h-6"
                    on:click=move |_| {
                        log::debug!("CLICK {}", zoom.get());
                        setZoom.set(zoom.get() - 1);
                    }
                >
                    "-"
                </button>
            </div>

            <div
                id="base"
                node_ref=div_ref
                class="z-0"
                style=move || format!("transform: translate3d({}px, {}px, 0px); opacity: 1;", x.get() - topleft.get().x , y.get() - topleft.get().y)
                style=move || format!("top:{}px; left:{}px", topleft.get().x , topleft.get().y)
                // style=move || format!("transform: translate3d({}px, {}px, 0px); opacity: 1;", x.get()  , y.get() )

                // on:click= move |ev: MouseEvent| {
                //     log::debug!("Clien::{:#?}:{:#?}", ev.client_x(), ev.client_y());
                //     log::debug!("Offset::{:#?}:{:#?}", ev.offset_x(), ev.offset_y());
                // }
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
                    style:width="6px"
                    style:height="6px"
                    style:background="red"
                ></div>
            </div>
            <div
                    class="absolute"
                    style:top="350px"
                    style:left="450px"
                    style:width="6px"
                    style:height="6px"
                    style:background="red"
                ></div>
            <div
                class="absolute"
                style:bottom="0px"
            >
                <p>Transform</p>
                <p>X: {move || x.get()}</p>
                <p>Y: {move || y.get()}</p>
                <p>boundX: {move || topleft.get().x}</p>
                <p>boundY: {move || topleft.get().y}</p>
            </div>


        </div>
        </div>
    }
}
