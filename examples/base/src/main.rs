use kapta::coords::KaptaCoord;
use leptos::*;
use leptos_kapta::Kapta;


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| leptos::view! { <App/> })
}

#[component]
pub fn App() -> impl IntoView {
    let center: KaptaCoord = KaptaCoord::new(106.645, 10.788);
    log::debug!("{:#?}",center);
    view! {
        <Kapta zoom=3 width=900 height=700 center=center/>
    }
}