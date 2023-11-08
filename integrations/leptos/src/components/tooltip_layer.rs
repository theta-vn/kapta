use kapta::views::Tooltip;
use leptos::{html::Div, *};

#[component]
pub fn TooltipLayer(tooltip: ReadSignal<Tooltip>) -> impl IntoView {
    let (html, set_html) = create_signal("".to_string());
    let (width, set_width) = create_signal(0.);
    let (height, set_height) = create_signal(0.);
    let div_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        set_html.set(tooltip.get().html);
        let bound = div_ref.get().unwrap().get_bounding_client_rect();
        // log::debug!("{:#?} x {:#?}", bound.width(), bound.height());
        set_width.set(bound.width());
        set_height.set(bound.height());
    });
    view! {
        <div id="kapta-popup">
            <div
                id="tool_span"
                node_ref=div_ref
                style="position: absolute; z-index: 100"
                style:top=move || format!("{}px", tooltip.get().coor.y - height.get() - 37.)
                style:left=move || format!("{}px", tooltip.get().coor.x - width.get() / 2.)
            >
                <div style="padding: 10px; border-radius: 10px; background: #fff; position: relative; z-index: 100; box-shadow: 0 3px 14px rgba(0,0,0,0.4); white-space: nowrap">
                    {move || html.get()}
                </div>
                <div style="width: 17px; height: 17px; transform: rotate(45deg); margin: auto; z-index: 101; margin-top: -8px; background: #fff; position: relative"></div>
            </div>

        </div>
    }
}
