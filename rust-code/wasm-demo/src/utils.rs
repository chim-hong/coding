use js_sys::{Array, Object};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::console::{self, log_1};

use crate::IndexType;



pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn object_to_array(obj: &Object, key_name: &IndexType) -> Array {
    log_1(&JsValue::from("data_1"));
    let binding = JsValue::from(obj);
    let object = binding.dyn_ref::<Object>().unwrap();
    let index_key = match key_name {
        IndexType::Key => Object::keys(object),
        IndexType::Value => Object::values(object),
        IndexType::Entries => Object::entries(object),
        _ => Object::entries(object),
    };
    Array::from(&index_key)
}
