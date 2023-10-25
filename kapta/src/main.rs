#[warn(unused_variables)]
use geo_types::Coord;
use kapta::k_view::{KCollection, KView};
use leptos::{html::Div, *};
use leptos_use::{use_draggable_with_options, UseDraggableOptions, UseDraggableReturn};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}
#[component]
pub fn App() -> impl IntoView {
    let center: Coord = (106.645, 10.788).into();
    view! {
        <Kapta zoom=3 width=900 height=700 center=center/>
    }
}

#[component]
pub fn Kapta(zoom: u8, width: u32, height: u32, center: Coord) -> impl IntoView {
    let (loading, set_loading) = create_signal(true);
    let (topleft, set_topleft) = create_signal(Coord { x: 0., y: 0. });
    let (zoom, set_zoom) = create_signal(zoom);
    let (view, set_view) = create_signal(KView::default());
    let (collection, set_collection) = create_signal(KCollection::default());

    let div_ref = create_node_ref::<Div>();

    let UseDraggableReturn {
        x, y, is_dragging, ..
    } = use_draggable_with_options(
        div_ref,
        UseDraggableOptions::default().prevent_default(true),
    );

    create_effect(move |_| {
        // Chi lay goc kapta lan dau
        if loading.get() {
            let bound = div_ref.get().unwrap().get_bounding_client_rect();
            set_topleft.set(Coord {
                y: bound.top(),
                x: bound.left(),
            });
            set_loading.set(false);
        }

        let kview = KView::new(center, topleft.get(), width, height, zoom.get());
        set_view.set(kview);
        let kcollection = view.get().new_collection();
        set_collection.set(kcollection);
        // // set_tiles.set(tview.to_vec_tile(w, h, topleft.get().x, topleft.get().y));
        log::debug!("{:#?}", view.get());
        log::debug!("{:#?}", collection.get());

        // {
        //     if is_dragging.get() {
        //         let length__hafl_tile = (2 as i64).pow((zoom.get() - 1).into());
        //         let pixel_x = BOUND_LON_3857 / 256. / length__hafl_tile as f64;
        //         let pixel_y = BOUND_LAT_3857 / 256. / length__hafl_tile as f64;

        //         let dproj = Coord {
        //             x: x.get() * pixel_x,
        //             y: y.get() * pixel_y,
        //         };
        //         let proj_3857 = KCoord::from(ct).to_proj_coord();
        //         let proj_3857_new = Coord {
        //             x: proj_3857.x - dproj.x,
        //             y: proj_3857.y - dproj.y,
        //         };
        //         // log::debug!("{:#?}:{:#?}", proj_3857, proj_3857_new);
        //         let bound_rec = bound_rec_tile(proj_3857_new, zoom.get(), w, h);
        //         let change = view.get().extent_bound(bound_rec);

        //         if change {
        //             // TODO: get new tile
        //             let tview = TView::load(proj_3857_new, zoom.get(), w, h);
        //             let ktiles = tview.to_k_tile_collect(w, h, Coord { x: topleft.get().x, y: topleft.get().y });

        //             log::debug!("{:#?}", ktiles);
        //             // set_view.set(tview);
        //         }
        //     }
        // }
    });

    view! {
        <div class="">
        <div
            class="mx-auto relative bg-primary-80 mt-8" // overflow-hidden
            // class="relative bg-primary-80" // overflow-hidden
            style:height=move || format!("{}px", height)
            style:width=move || format!("{}px", width)
        >
            <div id="control"
                class="top-0 right-0 z-50 absolute"
            >
                <button
                    class="bg-primary-80 block w-8 h-6"
                    on:click=move |_| {
                        log::debug!("CLICK {}", zoom.get());
                        set_zoom.set(zoom.get() + 1);

                    }
                >
                    "+"
                </button>
                <button
                    class="bg-primary-80 block w-8 h-6"
                    on:click=move |_| {
                        log::debug!("CLICK {}", zoom.get());
                        set_zoom.set(zoom.get() - 1);
                    }
                >
                    "-"
                </button>
            </div>

            <div
            node_ref=div_ref
                id="base"
                class="z-0 relative"

                // style:height=move || format!("{}px", h)
                // style:width=move || format!("{}px", w)
                style:transform=move || format!("translate3d({}px, {}px, 0px)", x.get() - topleft.get().x, y.get() - topleft.get().y)
            >

                {
                    move || view.get().to_img(collection.get()).iter().map(|(url, transform)| {
                        view! {
                            <img
                                alt=""
                                class="absolute top-0 left-0"
                                style="width: 256px; height: 256px; opacity: 1;"
                                src=url
                                style:transform=transform
                            />
                        }
                    })
                    .collect::<Vec<_>>()
                }


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

            // <div
            //     class="absolute"
            //     style:bottom="0px"
            //     style:right="0px"
            // >
            //     <p>Center: {move || format!("{:.2}#{:.2}",view.get().center.x, view.get().center.y)}</p>
            //     <p>TopLeft: {move || format!("{:.2}#{:.2}",view.get().top_left.x, view.get().top_left.y)}</p>
            //     <p>BotomRight: {move || format!("{:.2}#{:.2}",view.get().bottom_right.x, view.get().bottom_right.y)}</p>
            //     <p>is_dragging: {move || is_dragging.get()}</p>
            // </div>


        </div>
        </div>
    }
}
