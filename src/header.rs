mod hamburger_button;

use hamburger_button::HamburgerButton;

use leptos::*;

#[component]
pub fn Header(cx: Scope, header_height: u32) -> impl IntoView {
    view! { cx,
        <div id="header-container" style:height=format!("{}px", header_height)>
            <div id="logo-container">
                <img id="logo-svg" src="/laughing-man.svg"/>
                <img id="logo-text-svg" src="/logo_text.svg" />
            </div>
            <div id="hamburger-container">
                <div id="hamburger-text">"パラメータ―"</div>
                <HamburgerButton/>
            </div>
        </div>
    }
}
