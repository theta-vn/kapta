use leptos::*;

#[derive(PartialEq)]
enum ButtonZoom {
    Inc,
    Des,
}

#[component]
pub fn Control(zoom: ReadSignal<u8>, set_zoom: WriteSignal<u8>) -> impl IntoView {
    view! {
        <div id="kapta-control" style="position: absolute; z-index: 50; top: 0; right: 0;">
            <BZoom kind=ButtonZoom::Inc zoom=zoom set_zoom=set_zoom/>
            <BZoom kind=ButtonZoom::Des zoom=zoom set_zoom=set_zoom/>
        </div>
    }
}


#[component]
fn BZoom(kind: ButtonZoom, zoom: ReadSignal<u8>, set_zoom: WriteSignal<u8>) -> impl IntoView {
    let (zoom_in, set_zoom_in) = create_signal(true);
    let (zoom_out, set_zoom_out) = create_signal(true);
    create_effect(move |_| {
        if zoom.get() > 1 {
            set_zoom_in.set(true);
        } else {
            set_zoom_in.set(false);
        }
        if zoom.get() < 19 {
            set_zoom_out.set(true);
        } else {
            set_zoom_out.set(false);
        }
    });

    {
        return move || {
            if kind == ButtonZoom::Inc {
                view! {
                    <button
                        style="width: 40px; height: 32px; display: block;border: solid 1px;border-radius: 5px;margin: 5px;"
                        style:background=move || if zoom_out.get() { "white" } else { "gray" }
                        on:click=move |_| {
                            set_zoom.update(|n| *n = *n + 1);
                        }

                        disabled=!zoom_out.get()
                    >
                        <b>"+"</b>
                    </button>
                }
            } else {
                view! {
                    <button
                        style="width: 40px; height: 32px; display: block;border: solid 1px;border-radius: 5px;margin: 5px;"
                        style:background=move || if zoom_in.get() { "white" } else { "gray" }
                        on:click=move |_| {
                            set_zoom.update(|n| *n = *n - 1);
                        }

                        disabled=!zoom_in.get()
                    >
                        <b>"-"</b>
                    </button>
                }
            }
        };
    }
}