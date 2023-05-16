use leptos::*;

#[derive(Clone)]
pub struct LaughingManState {
    pub top: i32,
    pub left: i32,
    pub width: i32,
    pub height: i32,
}

#[component]
pub fn LaughingManImage(
    cx: Scope,
    #[prop(into)] id: String,
    state: ReadSignal<LaughingManState>,
) -> impl IntoView {
    let svg_str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/images/laughing-man.svg"
    ));

    view! {cx,
        <div
            id=id
            inner_html=svg_str
            style:top=move ||{format!("{}px",state.get().top)}
            style:left=move ||{format!("{}px",state.get().left)}
            style:width=move ||{format!("{}px",state.get().width)}
            style:height=move ||{format!("{}px",state.get().height)}
        ></div>
    }
}
