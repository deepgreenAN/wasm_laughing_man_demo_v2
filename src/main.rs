mod canvas_app;
mod error;
mod header;
mod reuse_interval;

use canvas_app::CanvasApp;
use header::Header;
pub use reuse_interval::Interval;

use leptos::*;

/// サイドメニューが開いているかどうか
#[derive(Clone, Debug, Copy)]
pub struct IsSideMenuActive(pub RwSignal<bool>);

#[component]
fn App(cx: Scope) -> impl IntoView {
    provide_context(cx, IsSideMenuActive(create_rw_signal(cx, false)));

    view! { cx,
        <div id="app-container">
            <Header header_height=100/>
            <CanvasApp
                header_height=100
            />
        </div>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    mount_to_body(|cx| view! {cx, <App/>})
}
