use super::{Control, GeoJsonLayer, TileLayer};
use geojson::FeatureCollection;
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
pub fn Kapta(
    zoom: u8,
    width: u32,
    height: u32,
    center: KaptaCoord,
    #[prop(default = None)] feature_collection: Option<FeatureCollection>,
    #[prop(default = 0)] preload: u8,
) -> impl IntoView {
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
        // First get top left of view
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
                let kview = KaptaView::new(
                    new_center.get(),
                    topleft.get(),
                    width,
                    height,
                    zoom.get(),
                    preload,
                );
                set_view.set(kview);

                // Drap end
                if position.get().x != 0. || position.get().y != 0. {
                    let kview = KaptaView::new(
                        new_center.get(),
                        topleft.get(),
                        width,
                        height,
                        zoom.get(),
                        preload,
                    );
                    set_view.set(kview);
                    set_position.set(Position::default());
                    // log::debug!("{:#?}", view.get());
                }

                let kcollection = view.get().new_collection();
                set_collection.set(kcollection);
            } else {
                let (check, top_left, center, bottom_right, proj_3857_new) = view
                    .get()
                    .drap_change_bound(position.get().x, position.get().y);
                set_new_center.set(proj_3857_new);
                if check {
                    let change = view.get().change_collection(bottom_right, center, top_left);
                    set_collection.set(change);
                }
            }
        }
    });

    view! {
        <div
            id="kapta"
            style="position: relative; overflow: hidden; background-color: aliceblue"
            style:height=move || format!("{}px", height)
            style:width=move || format!("{}px", width)
        >
            <Control zoom=zoom set_zoom=set_zoom/>
            <div
                node_ref=div_ref
                id="kapta-proxy"
                style="position: absolute; z-index: 90;"
                style:transform=move || {
                    format!("translate3d({}px, {}px, 0px)", -topleft.get().x, -topleft.get().y)
                }
            >

                <div

                    style="position: absolute; z-index: 90;"
                    style:height=move || format!("{}px", height)
                    style:width=move || format!("{}px", width)
                    style:transform=move || {
                        format!(
                            "translate3d({}px, {}px, 0px)",
                            position.get().x + topleft.get().x,
                            position.get().y + topleft.get().y,
                        )
                    }
                >
                </div>
            </div>
            <GeoJsonLayer
                feature_collection=feature_collection
                zoom=zoom
                view=view
                position=position
                is_dragging=is_dragging
            />
            <div

                id="kapta-base"
                style="position: relative; z-index: 0;"
                style:transform=move || {
                    format!(
                        "translate3d({}px, {}px, 0px)",
                        position.get().x - topleft.get().x,
                        position.get().y - topleft.get().y,
                    )
                }
            >

                <TileLayer view=view collection=collection/>

            </div>
        <div id="kapta-debug">
        <div
        style:position="absolute"
        style:top="345px"
        style:left="445px"
        style:width="10px"
        style:height="10px"
        style:background="red"
        style:border-radius="5px"
        >
        </div>
        <div
        style:position="absolute"
        style:bottom="0px"
        >
        <p>X: {move || position.get().x}</p>
        <p>Y: {move || position.get().y}</p>
        <p>Z: {move || zoom.get()}</p>
        <p>Center: {move || format!("{:.2}:{:.2}", new_center.get().coord.x,  new_center.get().coord.y)}</p>
        </div>

        <div
        style:position="absolute"
        style:bottom="0px"
        style:right="0px"
        >
        <p>Center: {move || format!("{:.2}#{:.2}",view.get().center.coord.x, view.get().center.coord.y)}</p>
        <p>TopLeft: {move || format!("{:.2}#{:.2} {:#}",view.get().top_left.coord.x, view.get().top_left.coord.y, view.get().top_left.coord.y.floor())}</p>
        <p>BotomRight: {move || format!("{:.2}#{:.2}",view.get().bottom_right.coord.x, view.get().bottom_right.coord.y)}</p>
        <p>is_dragging: {move || is_dragging.get()}</p>
        <p>loading: {move || loading.get()}</p>
        </div>
        </div>
        </div>
    }
}
