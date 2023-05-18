use super::dom_utils::document;
use crate::error::AppError;

use wasm_bindgen::JsCast;

/// 笑い男画像のオプション
#[derive(Clone, Debug)]
pub struct LaughingManOptions {
    pub laughing_man_size_ratio: f64,
    pub laughing_man_shift_ratio: f64,
    pub laughing_man_z_index: u32,
}

impl Default for LaughingManOptions {
    fn default() -> Self {
        LaughingManOptions {
            laughing_man_size_ratio: 1.0,
            laughing_man_shift_ratio: 0.0,
            laughing_man_z_index: 4,
        }
    }
}

/// 笑い男画像で使用する状態
#[derive(Clone, Debug)]
pub struct LaughingManState {
    pub id: u32,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

/// Signalのネストや多用を防ぐため，直接DOMを操作する笑い男画像
pub struct LaughingManImage {
    img_element: web_sys::HtmlImageElement,
    options: LaughingManOptions,
    id: u32,
}

impl LaughingManImage {
    pub fn new(
        parent_node: web_sys::Node,
        src_url: &str,
        options: LaughingManOptions,
        initial_state: LaughingManState,
    ) -> Result<Self, AppError> {
        let img_element = document()
            .create_element("img")?
            .dyn_into::<web_sys::HtmlImageElement>()
            .map_err(|element| {
                AppError::DomError(format!(
                    "Cannot convert from: {element:?} into {}",
                    std::any::type_name::<web_sys::HtmlImageElement>()
                ))
            })?;
        img_element.set_id(&format!("laughing-man-img-{}", initial_state.id));
        img_element
            .set_height(((initial_state.height as f64) * options.laughing_man_size_ratio) as u32); // 高さのみ合わせる

        let img_style = img_element.style();
        img_style.set_css_text(&format!(
            "position:absolute;top:{}px;left:{}px;z-index:{}",
            (options.laughing_man_shift_ratio * (initial_state.height as f64)
                + (initial_state.top as f64)) as u32,
            (options.laughing_man_shift_ratio * (initial_state.width as f64)
                + (initial_state.left as f64)) as u32,
            options.laughing_man_z_index
        ));
        img_element.set_src(src_url);

        parent_node.append_child(img_element.as_ref())?;

        Ok(Self {
            img_element,
            options,
            id: initial_state.id,
        })
    }

    pub fn step(&self, state: LaughingManState) -> Result<(), AppError> {
        self.img_element
            .set_height(((state.height as f64) * self.options.laughing_man_size_ratio) as u32); // 高さのみ合わせる
        let img_style = self.img_element.style();
        img_style.set_property(
            "top",
            &format!(
                "{}px",
                (self.options.laughing_man_shift_ratio * (state.height as f64) + (state.top as f64))
                    as u32
            ),
        )?;
        img_style.set_property(
            "left",
            &format!(
                "{}px",
                (self.options.laughing_man_shift_ratio * (state.width as f64) + (state.left as f64))
                    as u32
            ),
        )?;
        Ok(())
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for LaughingManImage {
    fn drop(&mut self) {
        self.img_element.remove();
    }
}
