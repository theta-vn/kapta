use geojson::FeatureCollection;
use kapta::{
    coords::{geojson_to_kaptageo, KaptaGeo, KaptaPoint, KaptaPolygon},
    views::KaptaView, consts::{BOUND_LON_3857, BOUND_LAT_3857},
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
    let (tmp_translate, set_tmp_translate) = create_signal([0.; 2]);
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
            // zoom else dragend
            
            if !draged.get() {
                log::debug!("zoom");
                let center_pixel = point_to_pixel(
                    [
                        view.get().center_p3857.coord.x,
                        view.get().center_p3857.coord.y,
                    ],
                    zoom.get(),
                );

                set_tmp_translate.set([
                    center_pixel[0] - (view.get().width as f64 / 2.),
                    center_pixel[1] - (view.get().height as f64 / 2.),
                ]);
                set_loading.set(false);
                set_translate_svg.set(tmp_translate.get());
            } else {
                let center_pixel = point_to_pixel(
                    [
                        view.get().center_p3857.coord.x,
                        view.get().center_p3857.coord.y,
                    ],
                    zoom.get(),
                );
                set_draged.set(false);
                set_tmp_translate.set([
                    center_pixel[0] - (view.get().width as f64 / 2.),
                    center_pixel[1] - (view.get().height as f64 / 2.),
                ]);
                // set_tmp_translate.set([
                //     tmp_translate.get()[0] - tmp_position.get()[0],
                //     tmp_translate.get()[1] - tmp_position.get()[1],
                // ]);
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

            set_tmp_translate.set([
                center_pixel[0] - (view.get().width as f64 / 2.),
                center_pixel[1] - (view.get().height as f64 / 2.),
            ]);
       
            set_loading.set(false);
            set_translate_svg.set(tmp_translate.get());
        }
        // When dragging
        if is_dragging.get() {
            // log::debug!("EFFECT DRAGGING tmp_translate::{:#?}", tmp_translate.get());
            set_translate_svg.set([
                tmp_translate.get()[0] - position.get().x,
                tmp_translate.get()[1] - position.get().y,
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
            <svg xmlns="http://www.w3.org/2000/svg" viewBox={move || format!("0 0 {} {}", view.get().width ,view.get().height)}
                style="position: absolute;top: 0px; left: 0px; z-index: 10;"
            >
                <g transform={move ||  format!("translate({},{})", -translate_svg.get()[0], -translate_svg.get()[1])}>
                <For
                    each=move || memo_data.get()
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
    // log::debug!("{:#?}", kp.value);
    let point = point_to_pixel(kp.value, zoom.get());
    // log::debug!("{:#?}", point);
    let d = format!("M {} {} l -7 -20 h 14 Z", point[0], point[1]);
    view! {
        <g>
            // <path
            //     fill="#ff0000"
            //     fill-rule="evenodd"
            //     d="M11.291 21.706 12 21l-.709.706zM12 21l.708.706a1 1 0 0 1-1.417 0l-.006-.007-.017-.017-.062-.063a47.708 47.708 0 0 1-1.04-1.106 49.562 49.562 0 0 1-2.456-2.908c-.892-1.15-1.804-2.45-2.497-3.734C4.535 12.612 4 11.248 4 10c0-4.539 3.592-8 8-8 4.408 0 8 3.461 8 8 0 1.248-.535 2.612-1.213 3.87-.693 1.286-1.604 2.585-2.497 3.735a49.583 49.583 0 0 1-3.496 4.014l-.062.063-.017.017-.006.006L12 21zm0-8a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"
            //     clip-rule="evenodd"
            // />
            <path d=d  stroke="red" fill="#ff0000"/>
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
    // [
    //     slide[0] * 256. * length_tile as f64 / (BOUND_LON_3857 * 2.), // 256.,
    //     slide[1] * 256. * length_tile as f64 / (BOUND_LAT_3857 * 2.)
    // ]
    [
        slide[0] * 128. * length_tile as f64 / (BOUND_LON_3857 ), // 256.,
        slide[1] * 128. * length_tile as f64 / (BOUND_LAT_3857 )
    ]
}
