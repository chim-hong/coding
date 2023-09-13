mod utils;

use js_sys::{Array, Object};
use serde::{Deserialize, Serialize};
use tsify::{self, Tsify};
use wasm_bindgen::prelude::*;
use web_sys::{console::log_1, window, Window};

#[derive(Tsify)]
pub enum IndexType {
    Key,
    Value,
    Entries,
}

#[derive(Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Point {
    #[tsify(type = "'key'|'value'|'entries'")]
    x: String,
    y: i32,
}

#[wasm_bindgen]
pub fn object_to_array(obj: Object, index_name: Point) -> Array {
    // utils::object_to_array(&obj, &index_name)
    log_1(&JsValue::from(index_name.x));
    Array::from(&JsValue::from(11))
}

#[wasm_bindgen]
pub fn log_title() {
    let title = window().unwrap().document().unwrap().title();
    log_1(&JsValue::from(title));
}
