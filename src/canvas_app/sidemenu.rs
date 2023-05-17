use crate::canvas_app::{CanvasAppOptions, LaughingManOptions, TrackerOptions};
use leptos::*;

use std::fmt::Debug;
use std::str::FromStr;

fn parse_input_value<T, E>(node_ref: NodeRef<leptos::html::Input>) -> T
where
    T: FromStr<Err = E>,
    E: Debug,
{
    node_ref
        .get()
        .expect("Cannot Get Input Element")
        .value()
        .parse()
        .expect("Cannot value Convert")
}

fn get_input_checked(node_ref: NodeRef<leptos::html::Input>) -> bool {
    node_ref.get().expect("Cannot get Input Element").checked()
}

fn set_input_value_as_number(node_ref: NodeRef<leptos::html::Input>, value: f64) {
    node_ref
        .get()
        .expect("Cannot Get Input Element")
        .set_value_as_number(value)
}

fn set_input_checked(node_ref: NodeRef<leptos::html::Input>, checked: bool) {
    node_ref
        .get()
        .expect("Cannot Get Input Element")
        .set_checked(checked)
}

#[component]
pub fn SideMenu<F>(cx: Scope, canvas_app_options: CanvasAppOptions, on_apply: F) -> impl IntoView
where
    F: Fn(CanvasAppOptions) + 'static,
{
    //　初期値
    let CanvasAppOptions {
        tracker_options,
        laughing_man_options,
        is_active_laughing_man,
        interval_span,
    } = canvas_app_options;

    // node_ref
    let image_over_video_scale_nr = create_node_ref::<leptos::html::Input>(cx);
    let min_face_size_nr = create_node_ref::<leptos::html::Input>(cx);
    let score_thresh_nr = create_node_ref::<leptos::html::Input>(cx);
    let pyramid_scale_factor_nr = create_node_ref::<leptos::html::Input>(cx);
    let slide_window_step_nr = create_node_ref::<leptos::html::Input>(cx);
    let is_active_laughing_man_nr = create_node_ref::<leptos::html::Input>(cx);
    let allowable_not_detect_count_nr = create_node_ref::<leptos::html::Input>(cx);
    let laughing_man_size_ratio_nr = create_node_ref::<leptos::html::Input>(cx);
    let laughing_man_shift_ratio_nr = create_node_ref::<leptos::html::Input>(cx);
    let interval_span_nr = create_node_ref::<leptos::html::Input>(cx);

    // apply, default関数
    let apply = move |_| {
        let tracker_options = TrackerOptions {
            image_over_video_scale: parse_input_value(image_over_video_scale_nr),
            min_face_size: parse_input_value(min_face_size_nr),
            score_thresh: parse_input_value(score_thresh_nr),
            pyramid_scale_factor: parse_input_value(pyramid_scale_factor_nr),
            slide_window_step: parse_input_value(slide_window_step_nr),
            allowable_not_detect_count: parse_input_value(allowable_not_detect_count_nr),
        };

        let laughing_man_options = LaughingManOptions {
            laughing_man_size_ratio: parse_input_value(laughing_man_size_ratio_nr),
            laughing_man_shift_ratio: parse_input_value(laughing_man_shift_ratio_nr),
            ..Default::default()
        };
        let is_active_laughing_man: bool = get_input_checked(is_active_laughing_man_nr);
        let interval_span: u32 = parse_input_value(interval_span_nr);

        let canvas_app_options = CanvasAppOptions {
            tracker_options,
            laughing_man_options,
            is_active_laughing_man,
            interval_span,
        };
        on_apply(canvas_app_options);
    };

    let default = move |_| {
        let CanvasAppOptions {
            tracker_options,
            laughing_man_options,
            is_active_laughing_man,
            interval_span,
        } = CanvasAppOptions::default();

        set_input_value_as_number(
            image_over_video_scale_nr,
            tracker_options.image_over_video_scale,
        );
        set_input_value_as_number(min_face_size_nr, tracker_options.min_face_size as f64);
        set_input_value_as_number(score_thresh_nr, tracker_options.score_thresh);
        set_input_value_as_number(
            pyramid_scale_factor_nr,
            tracker_options.pyramid_scale_factor as f64,
        );
        set_input_value_as_number(
            slide_window_step_nr,
            tracker_options.slide_window_step as f64,
        );
        set_input_checked(is_active_laughing_man_nr, is_active_laughing_man);
        set_input_value_as_number(
            allowable_not_detect_count_nr,
            tracker_options.allowable_not_detect_count as f64,
        );
        set_input_value_as_number(
            laughing_man_size_ratio_nr,
            laughing_man_options.laughing_man_size_ratio,
        );
        set_input_value_as_number(
            laughing_man_shift_ratio_nr,
            laughing_man_options.laughing_man_shift_ratio,
        );
        set_input_value_as_number(interval_span_nr, interval_span as f64);
    };

    view! {cx,
        <div id="side-menu-container">
            <label>
                "ビデオサイズに対する画像比[0.1, 1]:"
                <input type="number" min=0.1 max=1 step=0.01
                    value={tracker_options.image_over_video_scale.to_string()}
                    node_ref=image_over_video_scale_nr
                />
            </label>
            <label>
                "最小顔サイズ[20, ):"
                <input type="number" min=20 step=1
                    value={tracker_options.min_face_size.to_string()}
                    node_ref=min_face_size_nr
                />
            </label>
            <label>
                "スコア閾値(0, ):"
                <input type="number" min=0.01
                    value={tracker_options.score_thresh.to_string()}
                    node_ref=score_thresh_nr
                />
            </label>
            <label>
                "ピラミッドスケール[0.01, 0.99]:"
                <input type="number" min=0.01 max=0.99 step=0.01
                    value={tracker_options.pyramid_scale_factor.to_string()}
                    node_ref=pyramid_scale_factor_nr
                />
            </label>
            <label>
                "スライドウインドウステップサイズ[1, ):"
                <input type="number" min=1 step=1
                    value={tracker_options.slide_window_step.to_string()}
                    node_ref=slide_window_step_nr
                />
            </label>
            <label>
                "笑い男モード:"
                <input type="checkbox" checked={is_active_laughing_man}
                    node_ref=is_active_laughing_man_nr
                />
            </label>
            <label>
                "トラッキング許容カウント[1, ):"
                <input type="number" min=1
                    value={tracker_options.allowable_not_detect_count.to_string()}
                    node_ref=allowable_not_detect_count_nr
                />
            </label>
            <label>
                "笑い男拡大係数(0, ):"
                <input type="number" min=0 step=0.01
                    value={laughing_man_options.laughing_man_size_ratio.to_string()}
                    node_ref=laughing_man_size_ratio_nr
                />
            </label>
            <label>
                "笑い男シフト係数( , ):"
                <input type="number" step=0.01
                    value={laughing_man_options.laughing_man_shift_ratio.to_string()}
                    node_ref=laughing_man_shift_ratio_nr
                />
            </label>
            <label>
                "描画インターバル時間(10, ):"
                <input type="number" step=10
                    value={interval_span.to_string()}
                    node_ref=interval_span_nr
                />
            </label>
            <div id="default-apply-button">
                <button id="default-button" on:click=default>"デフォルト"</button>
                <button id="apply-button" on:click=apply>"適用"</button>
            </div>
            <a href="https://gist.github.com/johan/1066590" target="_blank" rel="noopener noreferrer">
                "画像引用元(Johan Sundströmさん)"
            </a>
            <div id="library-link">
                "顔認識には"
                <a href="https://github.com/atomashpolskiy/rustface" target="_blank" rel="noopener noreferrer">
                    "rustface"
                </a>
                "を利用しています(元ライブラリ"
                <a href="https://github.com/seetaface/SeetaFaceEngine/tree/master/FaceDetection" target="_blank" rel="noopener noreferrer">
                    "Seetaface"
                </a>
                ")"
            </div>
        </div>
    }
}
