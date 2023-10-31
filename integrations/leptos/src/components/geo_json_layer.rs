use geojson::FeatureCollection;
use kapta::{
    coords::{geojson_to_kaptageo, KaptaGeo, KaptaPoint, KaptaPolygon, SeriesKG},
    views::KaptaView,
};
use leptos::*;
use leptos_use::core::Position;

#[component]
pub fn GeoJsonLayer(
    view: ReadSignal<KaptaView>,
    zoom: ReadSignal<u8>,
    position: Signal<Position>,
    is_dragging: Signal<bool>,

    #[prop(default = None)] feature_collection: Option<FeatureCollection>,
) -> impl IntoView {
    let (loading, set_loading) = create_signal(true);
    let (data, set_data) = create_signal(SeriesKG::default());
    let (view_box, set_view_box) = create_signal(String::from("0 0 0 0"));
    let (translate_svg, set_translate_svg) = create_signal([0.; 2]);
    let (center, set_center) = create_signal([0.; 2]);
    let (new_center, set_new_center) = create_signal([0.; 2]);
    create_effect(move |_| {
        if loading.get() {
            log::debug!("INIT::{:#?}", view.get());
            let center_init = point_to_pixel(
                [
                    view.get().center_p3857.coord.x, // + (view.get().width as f64 / 2.),
                    view.get().center_p3857.coord.y, // + (view.get().height as f64 / 2.),
                ],
                zoom.get(),
            );
            set_center.set(center_init);
            set_new_center.set(center_init);
            set_loading.set(false);
            set_translate_svg.set(center_init);
            if let Some(collection) = &feature_collection {
                let kapta_geo = geojson_to_kaptageo(collection.clone());
                set_data.set(kapta_geo);
            }

            let view_box = format!("0 0 {} {}", view.get().width, view.get().height);
            set_view_box.set(view_box);
        }

        {
            // First || zoom || end drap ELSE drapping
            if !is_dragging.get() && !loading.get() {
                log::debug!("O DAu");

                // Drap end
                if position.get().x == 0. && position.get().y == 0. {
                    set_center.set(new_center.get());
                    set_translate_svg.set([
                        new_center.get()[0] - (view.get().width as f64 / 2.),
                        new_center.get()[1] - (view.get().height as f64 / 2.),
                    ]);
                    //     set_center.set(new_center.get());
                    log::debug!("END DRAG:: {:#?}", 1);
                    //     let translate_svg = format!(
                    //         "translate({},{})",
                    //         -center.get()[0] + (view.get().width as f64 / 2.) ,
                    //         -center.get()[1] + (view.get().height as f64 / 2.)
                    //     );
                    //     set_translate_svg.set(translate_svg);
                }
            } else {
                set_new_center.set([
                    center.get()[0] - position.get().x,
                    center.get()[1] - position.get().y,
                ]);
                log::debug!("DRAGGING:: {:#?}", new_center.get());
                set_translate_svg.set([
                    center.get()[0] - (view.get().width as f64 / 2.) - position.get().x,
                    center.get()[1] - (view.get().height as f64 / 2.) - position.get().y,
                ]);
                // let translate_svg = format!(
                //     "translate({},{})",
                //     -center.get()[0] + (view.get().width as f64 / 2.) + position.get().x,
                //     -center.get()[1] + (view.get().height as f64 / 2.) + position.get().y
                // );
                // set_translate_svg.set(translate_svg);
            }
        }
    });

    view! {
        <div id="kapta-geojson">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox={move || view_box.get()}
                style="position: absolute;top: 0px; left: 0px; z-index: 10; fill-opacity: 0"
            >
                <g transform={move ||  format!("translate(-{},-{})", translate_svg.get()[0], translate_svg.get()[1])}>
                <For
                    each=move || data.get()
                    key=move |state| (state.clone(), zoom.get())
                    let:data
                >

                    {
                        match data {
                            KaptaGeo::Point(point)=> render_point(point, zoom).into_view(),
                            KaptaGeo::Polygon(polygon) => render_polygon(polygon).into_view(),
                        }
                    }

                </For>
                </g>
            </svg>
        </div>
    }
}

pub fn render_point(kp: KaptaPoint, zoom: ReadSignal<u8>) -> impl IntoView {
    log::debug!("{:#?}", kp.value);
    let point = point_to_pixel(kp.value, zoom.get());
    log::debug!("{:#?}", point);
    let d = format!("M {} {} h 100 v 100 Z", point[0], point[1]);
    view! {
        <g>
            // <path
            //     fill="#ff0000"
            //     fill-rule="evenodd"
            //     d="M11.291 21.706 12 21l-.709.706zM12 21l.708.706a1 1 0 0 1-1.417 0l-.006-.007-.017-.017-.062-.063a47.708 47.708 0 0 1-1.04-1.106 49.562 49.562 0 0 1-2.456-2.908c-.892-1.15-1.804-2.45-2.497-3.734C4.535 12.612 4 11.248 4 10c0-4.539 3.592-8 8-8 4.408 0 8 3.461 8 8 0 1.248-.535 2.612-1.213 3.87-.693 1.286-1.604 2.585-2.497 3.735a49.583 49.583 0 0 1-3.496 4.014l-.062.063-.017.017-.006.006L12 21zm0-8a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"
            //     clip-rule="evenodd"
            // />
            // <path
            //     // fill-rule="evenodd"
            //     fill="#ff0000"
            //     d="M11 291 l20,0 l0,20 z"
            // />
            // <path d="M 1 1 h 80 Z" fill="#ff0000" stroke="red"/>
            <path d=d fill="none" stroke="red"/>
        </g>
    }
}

pub fn render_polygon(polygon: KaptaPolygon) -> impl IntoView {
    // log::debug!("{:#?}", polygon);
    view! {
        <g>
        </g>
    }
}

pub fn point_to_pixel(slide: [f64; 2], zoom: u8) -> [f64; 2] {
    // log::debug!("{:#?}",slide);
    let length_tile = (2 as u64).pow(zoom.into());
    [
        slide[0] / length_tile as f64 / 256.,
        slide[1] / length_tile as f64 / 256.,
    ]
}
