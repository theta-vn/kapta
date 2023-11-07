use kapta::views::Tooltip;
use leptos::*;

#[component]
pub fn TooltipLayer(tooltip: ReadSignal<Tooltip>) -> impl IntoView {
    let (html, set_html) = create_signal("".to_string());
    create_effect(move |_| {
        set_html.set(tooltip.get().html);
    });
    view! {
        <div id="kapta-popup">
            {move || {
                if html.get() != "".to_string() {
                    view! {
                        <span
                            style="border: 1px solid #000; padding: 10px; border-radius: 10px; position: relative; background: #fff; z-index: 100"
                            style:top=move || format!("{}px", tooltip.get().coor.y)
                            style:left=move || format!("{}px", tooltip.get().coor.x)
                        >
                            {move || html.get()}
                        </span>
                    }
                } else {
                    view! { <span></span> }
                }
            }}

        </div>
    }
}
