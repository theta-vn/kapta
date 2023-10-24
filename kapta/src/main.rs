use geo_types::Coord;
use kapta::view;
use leptos::{html::Div, *};
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let (loading, setLoading) = create_signal(true);
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

        // Chi lay goc kapta lan dau
        if loading.get() {
            let bound = div_ref.get().unwrap().get_bounding_client_rect();
            setTopleft.set(Coord {
                y: bound.top(),
                x: bound.left(),
            });
            setLoading.set(false);
        }

        let view = view::render(w, h, zoom.get(), ct);
        setCenter.set(view.center);
        setArray.set(view.array);
        
        
        log::debug!("EFFECT:: {:#?}", array.get());
    });

    let UseDraggableReturn {
        x, y, is_dragging, ..
    } = use_draggable_with_options(
        div_ref,
        UseDraggableOptions::default().prevent_default(true),
    );

    view! {
        <div class="">
        <div
            class="mx-auto relative bg-primary-80 mt-8" // overflow-hidden 
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
            node_ref=div_ref
                id="base"
                class="z-0 relative"

                // style:top=move || format!("top:{}px", topleft.get().x)
                // style:top=move || format!("{}px", topleft.get().y)
                // style:left=move || format!("{}px", topleft.get().x)
                style:height=move || format!("{}px", h)
                style:width=move || format!("{}px", w)
                style:transform=move || format!("translate3d({}px, {}px, 0px)", x.get() - topleft.get().x, y.get() - topleft.get().y)
            >

                {move || array.get().iter().map(|data| {
                        let trans_x = (w / 2 - 128) as f64
                            + (data.1 as f64 + 0.5 - center.get().x) * 255. + topleft.get().x;
                        let trans_y = (h / 2 - 128) as f64
                            + (data.2 as f64 + 0.5 - center.get().y) * 255. + topleft.get().y;
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
                                style="width: 256px; height: 256px; opacity: 1;"
                                style:transform=trans.clone()
                            />
                        }
                    })
                    .collect::<Vec<_>>()}


            </div>
            <div
                    class="absolute"
                    style:top="345px"
                    style:left="445px"
                    style:width="10px"
                    style:height="10px"
                    style:background="red"
                    style:border-radius="5px"
                ></div>            
            <div
                    class="absolute"
                    style:top="0"
                    style:left="0"
                    style:width="900px"
                    style:height="700px"                    
                    style:border="solid"
                ></div>
            <div
                class="absolute"
                style:bottom="0px"
            >
                <p>X: {move || x.get()}</p>
                <p>Y: {move || y.get()}</p>
                <p>tlX: {move || topleft.get().x}</p>
                <p>ltY: {move || topleft.get().y}</p>
            </div>

            <div
                class="absolute"
                style:bottom="0px"
                style:right="0px"
            >
                <p>CenterX: {move || center.get().x}</p>
                <p>CenterY: {move || center.get().y}</p>

                <p>is_dragging: {move || is_dragging.get()}</p>
            </div>


        </div>
        </div>
    }
}
