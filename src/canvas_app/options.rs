#[derive(Clone, Debug)]
pub struct AppOption {
    pub video_canvas_parent_id: String,
    pub canvas_app_id: String,
    pub make_image_canvas_id: String,
    pub video_id: String,
    pub model_url: String,
    pub laughing_man_svg_path: String,
    pub image_over_video_scale: f64,
    pub min_face_size: u32,
    pub score_thresh: f64,
    pub pyramid_scale_factor: f32,
    pub slide_window_step: u32,
    pub is_active_laughing_man: bool,
    pub allowable_not_detect_count: u32,
    pub laughing_man_size_ratio: f64,
    pub laughing_man_shift_ratio: f64,
    pub laughing_man_z_index: u32,
}

#[derive(Clone, Debug)]
pub struct ModifyOption {
    pub image_over_video_scale: f64,
    pub min_face_size: u32,
    pub score_thresh: f64,
    pub pyramid_scale_factor: f32,
    pub slide_window_step: u32,
    pub is_active_laughing_man: bool,
    pub allowable_not_detect_count: u32,
    pub laughing_man_size_ratio: f64,
    pub laughing_man_shift_ratio: f64,
}
