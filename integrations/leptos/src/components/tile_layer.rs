use kapta::views::{KaptaView, SeriesPC};
use leptos::*;
#[component]
pub fn TileLayer(view: ReadSignal<KaptaView>, collection: ReadSignal<SeriesPC>) -> impl IntoView {
    view! {
        
        {move || {
            view.get()
                .to_img(collection.get())
                .iter()
                .map(|(url, transform)| {
                    view! {
                        <img
                            src=url
                            loading="lazy"
                            style:transform=transform
                            style="position: absolute;top: 0px; left: 0px; width: 256px; height: 256px; opacity: 1;"
                        />
                    }
                })
                .collect::<Vec<_>>()
        }}
        
    }
}
