use crate::IsSideMenuActive;

use leptos::*;

#[component]
pub fn HamburgerButton(cx: Scope) -> impl IntoView {
    let is_active = use_context::<IsSideMenuActive>(cx).expect("Cannot Get Context");

    let class = move || {
        if is_active.0.get() {
            "hamburger-button active"
        } else {
            "hamburger-button"
        }
    };

    let onclick = move |_| {
        is_active.0.update(|flag| *flag = !*flag);
    };

    view! {cx,
        <div class=class on:click=onclick>
            <svg width=32 height=24>
                <line id="top" x1=0 y1=2  x2=32 y2=2/>
                <line id="middle" x1=0 y1=12 x2=32 y2=12/>
                <line id="bottom" x1=0 y1=22 x2=32 y2=22/>
            </svg>
        </div>
    }
}
