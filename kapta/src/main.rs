use geo_types::{Coord, Point};
use kapta::view;
use leptos::{html::Div, *};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let h: u32 = 700;
    let w: u32 = 900;
    let zoom: u8 = 0;
    
    let center: Coord = (106.645, 10.788).into();
    let view = view::render(w, h, zoom, center);
    let node_ref = create_node_ref::<Div>();
    let (width, setWidth) = create_signal(0);
    let (height, setHeight) = create_signal(0);

    create_effect(move |_| {
        // let node = node_ref.get().unwrap();
        if let Some(node) = node_ref.get() {
            setWidth.set(node.client_width());
            setHeight.set(node.client_height());
            log::debug!("{}", height.get());
            log::debug!("{}", width.get());

            // log::debug!("Count::{}", count);
        }
    });
    log::debug!("{:#?}", view);
    view! {
        <div
            class="mx-auto relative overflow-hidden bg-primary-80"
            style:height=move || format!("{}px", h)
            style:width=move || format!("{}px", w)
        >
            // <div class="absolute top-0 left-0">
            // <div ref=node_ref class="absolute top-0 left-0"
            <div ref=node_ref class="top-0 left-0"
              style="transform: translate3d(0px, 0px, 0px); opacity: 1;"
            >

                {view
                    .array
                    .iter()
                    .map(|data| {
                        let trans_x = (w/2 - 128) as f64 + (data.1 as f64 +0.5 - view.center.x ) * 255.;
                        let trans_y = (h/2 - 128) as f64 + (data.2 as f64 +0.5 - view.center.y) * 255.;
                        let trans = format!("translate3d({}px, {}px, 0px)", trans_x , trans_y) ;
                        let count = (2 as i64).pow(zoom.into());
                        let url = format!(
                            "https://tile.openstreetmap.org/{}/{}/{}.png",
                            data.0,
                            (data.1 as i64).rem_euclid(count),
                            (data.2 as i64 % count)
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

                // <img alt="" src=url
                // class=""
                // style="width: 256px; height: 256px; opacity: 1;"
                // style:transform={trans}
                // />
                <div class="absolute" style:top="350px" style:left="450px" style:width="2px" style:height="2px"
                style:background="red"
                ></div>
            </div>
        // </div>
        </div>
    }
}
