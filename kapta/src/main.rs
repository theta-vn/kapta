// use leptos::ev::Event;
use leptos::{html::Div, *};
// use web_sys::{HtmlElement, HtmlDivElement};
// use web_sys::HtmlDivElement;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let h = 700;
    let w = 700;
    // let lat = 107.;
    // let long = 17.;
    let zoom = 3;
    let count = i32::pow(2, zoom);

    let node_ref = create_node_ref::<Div>();
    let (width, setWidth) = create_signal(0);
    let (height, setHeight) = create_signal(0);
    let mut array: Vec<String> = [].to_vec();
    for n in 0..count {
        for m in 0..count {
            let url = format!("https://tile.openstreetmap.org/{}/{}/{}.png", zoom, n, m);
            // log::debug!("URL::{}", url);
            array.push(url);
        }
    }
    log::debug!("{:#?}", array);
    create_effect(move |_| {
        // let node = node_ref.get().unwrap();
        if let Some(node) = node_ref.get() {
            setWidth.set(node.client_width());
            setHeight.set(node.client_height());
            log::debug!("{}", height.get());
            log::debug!("{}", width.get());

            log::debug!("Count::{}", count);
        }
    });

    view! {
      <div class="mx-auto">
        <div ref=node_ref class="bg-primary-80 flex"

          style:height=move || format!("{}px", h)
          style:width=move || format!("{}px", w)
        >
          {
            array.iter().map(|data| {
              view!{
                <img alt="" src={data.clone()}
                  class=""
                  style="width: 256px; height: 256px;" // transform: translate3d(402px, 213px, 0px); opacity: 1;" 
                />
              }

            }).collect::<Vec<_>>()
          }


        </div>
      </div>
    }
}
