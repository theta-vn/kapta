use geo_types::{Point, Coord};
use leptos::{html::Div, *};
use kapta::view;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let h: u32 = 700;
    let w: u32 = 900;    
    let zoom: u8 = 5;    
    let center: Coord = (107., 17.).into();
    let view = view::render(w, h, zoom, center);
    let node_ref = create_node_ref::<Div>();
    let (width, setWidth) = create_signal(0);
    let (height, setHeight) = create_signal(0);
    
   
    let trans = format!("translate3d({}px, {}px, 0px)", view.3, view.4);
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

    view! {
      <div class="mx-auto relative overflow-hidden bg-primary-80"
        style:height=move || format!("{}px", h)
        style:width=move || format!("{}px", w)
      >
        // <div class="absolute top-0 left-0">
          // <div ref=node_ref class="absolute top-0 left-0"
          <div ref=node_ref class="top-0 left-0"
            // style:height=move || format!("{}px", h)
            // style:width=move || format!("{}px", w)
          >
            {
              view.5.iter().map(|data| {
                view!{
                  <img alt="" src={data.clone()}
                    class="absolute top-0 left-0"
                    style="width: 256px; height: 256px; opacity: 1;" // transform: translate3d(402px, 213px, 0px); opacity: 1;"
                    style:transform={trans.clone()}
                  />
                }

              }).collect::<Vec<_>>()
            }
            // <img alt="" src=url
            //   class=""
            //   style="width: 256px; height: 256px; opacity: 1;"
            //   style:transform={trans}
            // />
            <hr 
              class="absolute "
              style:top={"350px"}
            />
          </div>
        // </div>
      </div>
    }
}
