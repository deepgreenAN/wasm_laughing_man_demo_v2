use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "setInterval", catch)]
    fn set_interval(handler: &Function, timeout: i32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "clearInterval")]
    fn clear_interval(handle: JsValue) -> JsValue;

}

#[derive(Debug, Default)]
pub struct Interval {
    id: Option<JsValue>,
    closure: Option<Closure<dyn FnMut()>>,
}

/// clear_intervalで削除
impl Drop for Interval {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            clear_interval(id);
        }
    }
}

impl Interval {
    pub fn new<F>(millis: u32, callback: F) -> Interval
    where
        F: 'static + FnMut(),
    {
        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);

        let id = set_interval(
            closure.as_ref().unchecked_ref::<js_sys::Function>(),
            millis as i32,
        )
        .unwrap_throw();

        Interval {
            id: Some(id),
            closure: Some(closure),
        }
    }

    pub fn forget(mut self) -> JsValue {
        let id = self.id.take().unwrap_throw();
        self.closure.take().unwrap_throw().forget();
        id
    }

    pub fn cancel(mut self) -> Closure<dyn FnMut()> {
        self.closure.take().unwrap_throw()
    }

    pub fn from_closure(millis: u32, closure: Closure<dyn FnMut()>) -> Self {
        let id = set_interval(
            closure.as_ref().unchecked_ref::<js_sys::Function>(),
            millis as i32,
        )
        .unwrap_throw();

        Interval {
            id: Some(id),
            closure: Some(closure),
        }
    }
}
