pub use kapta::coords::KaptaCoord;
use kapta::{
    coords::{Coord, ProjCoord},
    views::{KaptaView, SeriesPC},
};
use leptos::{html::Div, *};
use leptos_use::{
    core::Position, use_draggable_with_options, UseDraggableOptions, UseDraggableReturn,
};

#[component]
pub fn Kapta(zoom: u8, width: u32, height: u32, center: KaptaCoord) -> impl IntoView {
    let (loading, set_loading) = create_signal(true);
    let (topleft, set_topleft) = create_signal(Coord { x: 0., y: 0. });
    let (zoom, set_zoom) = create_signal(zoom);
    let (view, set_view) = create_signal(KaptaView::default());
    let (collection, set_collection) = create_signal(SeriesPC::default());
    let (new_center, set_new_center) = create_signal(ProjCoord::default());

    let div_ref = create_node_ref::<Div>();

    let UseDraggableReturn {
        position,
        set_position,
        is_dragging,
        ..
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

            let center_proj = ProjCoord::from(center);
            set_new_center.set(center_proj);
            set_loading.set(false);
        }

        {
            // First || zoom || end drap ELSE drapping
            if !is_dragging.get() && !loading.get() {
                let kview =
                    KaptaView::new(new_center.get(), topleft.get(), width, height, zoom.get());
                set_view.set(kview);

                // Drap end
                if position.get().x != 0. || position.get().y != 0. {
                    let kview =
                        KaptaView::new(new_center.get(), topleft.get(), width, height, zoom.get());
                    set_view.set(kview);
                    set_position.set(Position::default());
                }

                let kcollection = view.get().new_collection();
                set_collection.set(kcollection);
            } else {
                let (check, top_left, center, bottom_right, new_center) = view
                    .get()
                    .drap_change_bound(position.get().x, position.get().y);

                set_new_center.set(new_center);
                if check {
                    let change = view.get().change_collection(bottom_right, center, top_left);

                    set_collection.set(change);
                }
            }
        }
    });

    view! {

        <div
            style="position: relative; overflow: hidden; background-color: gray"
            style:height=move || format!("{}px", height)
            style:width=move || format!("{}px", width)
        >
            <div id="kapta-control"
                style="position: absolute; z-index: 50; top: 0; right: 0;"
            >
                <button
                    style="width: 30px; height: 30px; background-color: gray; display: block"
                    on:click=move |_| {
                        log::debug!("CLICK {}", zoom.get());
                        set_zoom.set(zoom.get() + 1);
                    }
                >
                    "+"
                </button>
                <button
                    style="width: 30px; height: 30px; background-color: gray; display: block"
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
                id="kapta-base"
                style="position: relative; z-index: 0;"
                style:transform=move || format!("translate3d({}px, {}px, 0px)", position.get().x - topleft.get().x, position.get().y - topleft.get().y)
            >
                {
                    move || view.get().to_img(collection.get()).iter().map(|(url, transform)| {
                        view! {
                            <img
                                src=url
                                style:transform=transform
                                style="position: absolute;top: 0px; left: 0px; width: 256px; height: 256px; opacity: 1;"
                            />
                        }
                    })
                    .collect::<Vec<_>>()
                }
            </div>
            // <div
            //         style:position="absolute"
            //         style:top="345px"
            //         style:left="445px"
            //         style:width="10px"
            //         style:height="10px"
            //         style:background="red"
            //         style:border-radius="5px"
            //     ></div>
            // <div
            //     style:position="absolute"
            //     style:bottom="0px"
            // >
            //     <p>X: {move || position.get().x}</p>
            //     <p>Y: {move || position.get().y}</p>
            //     <p>Z: {move || zoom.get()}</p>
            // </div>

            // <div
            //     style:position="absolute"
            //     style:bottom="0px"
            //     style:right="0px"
            // >
            //     <p>Center: {move || format!("{:.2}#{:.2}",view.get().center.coord.x, view.get().center.coord.y)}</p>
            //     <p>TopLeft: {move || format!("{:.2}#{:.2}",view.get().top_left.coord.x, view.get().top_left.coord.y)}</p>
            //     <p>BotomRight: {move || format!("{:.2}#{:.2}",view.get().bottom_right.coord.x, view.get().bottom_right.coord.y)}</p>
            //     <p>is_dragging: {move || is_dragging.get()}</p>
            // </div>
        </div>

    }
}
