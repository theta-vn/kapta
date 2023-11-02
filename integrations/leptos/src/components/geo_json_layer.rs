use geojson::FeatureCollection;
use kapta::{
    consts::{BOUND_LAT_3857, BOUND_LON_3857},
    coords::{geojson_to_kaptageo, KaptaGeo, KaptaPoint, KaptaPolygon},
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
    let (draged, set_draged) = create_signal(false);
    let (translate_svg, set_translate_svg) = create_signal([0.; 2]);
    let (translate, set_translate) = create_signal([0.; 2]);
    // let (tmp_position, set_tmp_position) = create_signal([0.; 2]);

    let memo_data = create_memo(move |_| {
        if let Some(collection) = &feature_collection {
            geojson_to_kaptageo(collection.clone())
        } else {
            [].to_vec()
        }
    });

    create_effect(move |_| {
        // Zoom || dragend
        if !loading.get() && !is_dragging.get() {
            // Dragend
            if !draged.get() {
                let center_pixel = point_to_pixel(
                    [
                        view.get().center_p3857.coord.x,
                        view.get().center_p3857.coord.y,
                    ],
                    zoom.get(),
                );

                set_translate.set([
                    center_pixel[0] - (view.get().width as f64 / 2.),
                    center_pixel[1] - (view.get().height as f64 / 2.),
                ]);
                set_loading.set(false);
                set_translate_svg.set(translate.get());
            } 
            // Zoom
            else {
                let center_pixel = point_to_pixel(
                    [
                        view.get().center_p3857.coord.x,
                        view.get().center_p3857.coord.y,
                    ],
                    zoom.get(),
                );
                set_draged.set(false);
                set_translate.set([
                    center_pixel[0] - (view.get().width as f64 / 2.),
                    center_pixel[1] - (view.get().height as f64 / 2.),
                ]);            
            }
        }

        // First
        if loading.get() {
            let center_pixel = point_to_pixel(
                [
                    view.get().center_p3857.coord.x,
                    view.get().center_p3857.coord.y,
                ],
                zoom.get(),
            );

            set_translate.set([
                center_pixel[0] - (view.get().width as f64 / 2.),
                center_pixel[1] - (view.get().height as f64 / 2.),
            ]);

            set_loading.set(false);
            set_translate_svg.set(translate.get());
        }
        // When dragging
        if is_dragging.get() {
            // log::debug!("EFFECT DRAGGING tmp_translate::{:#?}", tmp_translate.get());
            set_translate_svg.set([
                translate.get()[0] - position.get().x,
                translate.get()[1] - position.get().y,
            ]);

            if position.get().x == 0. && position.get().y == 0. {
                set_draged.set(false)
            } else {
                set_draged.set(true)
            }
        }
    });

    view! {
        <div id="kapta-geojson">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox=move || format!("0 0 {} {}", view.get().width, view.get().height)
                style="position: absolute;top: 0px; left: 0px; z-index: 10;"
            >
                <g transform=move || {
                    format!("translate({},{})", -translate_svg.get()[0], -translate_svg.get()[1])
                }>
                    <For
                        each=move || memo_data.get()
                        key=move |state| (state.clone(), zoom.get())
                        let:data
                    >

                        {match data {
                            KaptaGeo::Point(point) => render_point(point, zoom).into_view(),
                            KaptaGeo::Polygon(polygon) => render_polygon(polygon, zoom).into_view(),
                        }}

                    </For>
                </g>
            </svg>
        </div>
    }
}

pub fn render_point(kp: KaptaPoint, zoom: ReadSignal<u8>) -> impl IntoView {
    let point = point_to_pixel(kp.value, zoom.get());
    let d = format!("M{},{} l-9,-25 l5,-5 h8 l5,5Z", point[0], point[1]);
    view! {
        <g>
            <path d=d stroke="red" fill="#ff0000"></path>
        </g>
    }
}

pub fn render_polygon(polygon: KaptaPolygon, zoom: ReadSignal<u8>) -> impl IntoView {
    let hull = &polygon.value[0];
    let mut d = "M".to_string();
    for p in hull {
        let point = point_to_pixel(*p, zoom.get());

        let v = format!(" {},{} ", point[0], point[1]);
        d.push_str(&v);
    }
    d.push_str("Z");
    view! {
        <g>
            <path d=d stroke="red" fill="none"></path>
        </g>
    }
}

pub fn point_to_pixel(slide: [f64; 2], zoom: u8) -> [f64; 2] {
    let length_tile = (2 as u64).pow(zoom.into());
    [
        slide[0] * 128. * length_tile as f64 / (BOUND_LON_3857),
        slide[1] * 128. * length_tile as f64 / (BOUND_LAT_3857),
    ]
}
