mod canvas_app;
mod error;

use canvas_app::CanvasApp;

use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <CanvasApp />
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App/>})
}
