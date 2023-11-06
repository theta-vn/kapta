use geojson::FeatureCollection;
use kapta::{
    consts::{BOUND_LAT_3857, BOUND_LON_3857},
    coords::{geojson_to_kaptageo, KaptaGeo, KaptaLineString, KaptaPoint, KaptaPolygon},
    views::KaptaView,
};
use leptos::{svg::G, *};
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
    let (draped, set_draped) = create_signal(false);
    let (translate_svg, set_translate_svg) = create_signal([0.; 2]);
    let (tmp_translate, set_tmp_translate) = create_signal([0.; 2]);
    let (translate, set_translate) = create_signal(KaptaPoint::default());
    let memo_data = create_memo(move |_| {
        if let Some(collection) = &feature_collection {
            geojson_to_kaptageo(collection.clone())
        } else {
            [].to_vec()
        }
    });
    let svg_ref = create_node_ref::<G>();

    create_effect(move |_| {
        // Zoom || dragend
        zoom.get();
        if !loading.get() && !is_dragging.get() {
            if draped.get() {
                let bound = svg_ref.get().unwrap();
                let data_x = bound.get_attribute("data-x").unwrap();
                let data_y = bound.get_attribute("data-y").unwrap();

                let x = data_x.parse::<f64>().unwrap();
                let y = data_y.parse::<f64>().unwrap();
                set_tmp_translate.set([x, y]);
                set_draped.set(false);
            } else {
                let center_pixel = point_to_pixel(
                    [
                        view.get().center_p3857.coord.x,
                        view.get().center_p3857.coord.y,
                    ],
                    zoom.get(),
                );

                set_translate.set(KaptaPoint::from([
                    center_pixel[0] - (view.get().width as f64 / 2.),
                    center_pixel[1] - (view.get().height as f64 / 2.),
                ]));
                // set_loading.set(true);
                set_translate_svg.set([0., 0.]);
                set_tmp_translate.set([0., 0.]);
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

            set_translate.set(KaptaPoint::from([
                center_pixel[0] - (view.get().width as f64 / 2.),
                center_pixel[1] - (view.get().height as f64 / 2.),
            ]));

            set_loading.set(false);
            set_translate_svg.set([0., 0.]);
            set_tmp_translate.set([0., 0.]);
        }
        // When dragging
        if is_dragging.get() {
            set_translate_svg.set([
                -(position.get().x + tmp_translate.get()[0]),
                -(position.get().y + tmp_translate.get()[1]),
            ]);
            if position.get().x == 0. && position.get().y == 0. {
                set_draped.set(false);
            } else {
                set_draped.set(true);
            }
        }
    });

    view! {
        <div id="kapta-geojson">
            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox=move || format!("0 0 {} {}", view.get().width, view.get().height)
                style="position: absolute;top: 0px; left: 0px; z-index: 10;"
                style:height=move || format!("{}px", view.get().height)
                style:width=move || format!("{}px", view.get().width)
            >
                <g
                    id="kapta_svg"
                    node_ref=svg_ref
                    data-x=move || -translate_svg.get()[0]
                    data-y=move || -translate_svg.get()[1]
                    transform=move || {
                        format!(
                            "translate({},{})",
                            -translate_svg.get()[0],
                            -translate_svg.get()[1],
                        )
                    }
                >

                    <For
                        each=move || memo_data.get()
                        key=move |state| (state.clone(), zoom.get(), translate.get())
                        let:data
                    >

                        {match data {
                            KaptaGeo::Point(point) => {
                                render_point(point, zoom, translate).into_view()
                            }
                            KaptaGeo::MultiPoint(multi_point) => {
                                (multi_point
                                    .into_iter()
                                    .map(|n| render_point(n, zoom, translate).into_view())
                                    .collect::<Vec<_>>())
                                    .into_view()
                            }
                            KaptaGeo::Polygon(polygon) => {
                                render_polygon(polygon, zoom, translate).into_view()
                            }
                            KaptaGeo::LineString(line_string) => {
                                render_line_string(line_string, zoom, translate).into_view()
                            }
                            KaptaGeo::MultiLineString(multi_line_string) => {
                                (multi_line_string
                                    .into_iter()
                                    .map(|n| render_line_string(n, zoom, translate).into_view())
                                    .collect::<Vec<_>>())
                                    .into_view()
                            }
                            KaptaGeo::MultiPolygon(multi_polygon) => {
                                (multi_polygon
                                    .into_iter()
                                    .map(|n| render_polygon(n, zoom, translate).into_view())
                                    .collect::<Vec<_>>())
                                    .into_view()
                            }
                        }}

                    </For>
                </g>
            </svg>
        </div>
    }
}

pub fn render_point(
    point: KaptaPoint,
    zoom: ReadSignal<u8>,
    translate: ReadSignal<KaptaPoint>,
) -> impl IntoView {    
    let draw = match point.properties {
        Some(prop) => match prop.get("kapta") {
            Some(value) => {
                if value.is_object() {
                    if value["draw"] == "marker" {
                        "marker"
                    } else {
                        "circle"
                    }
                } else {
                    "circle"
                }
            }
            None => "circle",
        },
        None => "circle",
    };
    let point = point_sub(
        point_to_pixel(point.value, zoom.get_untracked()),
        translate.get_untracked().value,
    );
    if draw == "marker" {
        let d = format!(
            "M{},{}l-10,-25a10,10 1 0 1 20,0z m-5,-25 a5,5 1 0 1 10,0 a5,5 1 0 1 -10,0 z",
            point[0], point[1]
        );
        view! {
            <g>
                <path d=d stroke="none" fill="red" fill-rule="evenodd"></path>
            </g>
        }
    } else {
        view! {
            <g>
                <circle cx=point[0] cy=point[1] r="5" stroke="none" fill="red"></circle>
            </g>
        }
    }
}

pub fn render_polygon(
    polygon: KaptaPolygon,
    zoom: ReadSignal<u8>,
    translate: ReadSignal<KaptaPoint>,
) -> impl IntoView {
    let hull = &polygon.value[0];
    let mut d = "M".to_string();
    for p in hull {
        let point = point_sub(point_to_pixel(*p, zoom.get()), translate.get().value);

        let v = format!(" {},{} ", point[0], point[1]);
        d.push_str(&v);
    }
    d.push_str("Z");
    view! {
        <g>
            <path d=d stroke="blue" fill="none"></path>
        </g>
    }
}

pub fn render_line_string(
    line_string: KaptaLineString,
    zoom: ReadSignal<u8>,
    translate: ReadSignal<KaptaPoint>,
) -> impl IntoView {
    let hull = &line_string.value;
    let mut d = "M".to_string();
    for p in hull {
        let point = point_sub(point_to_pixel(*p, zoom.get()), translate.get().value);

        let v = format!(" {},{} ", point[0], point[1]);
        d.push_str(&v);
    }
    // d.push_str("Z");
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

pub fn point_sub(slide: [f64; 2], other: [f64; 2]) -> [f64; 2] {
    [slide[0] - other[0], slide[1] - other[1]]
}
