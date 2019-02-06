use console_error_panic_hook::set_once as set_panic_hook_once;
use wasm_bindgen::prelude::*;

use balena_temen as temen;

/// Evaluates JSON with `$$formula` keywords
///
/// # Arguments
///
/// * `data` - A JSON object
///
/// # Throws
///
/// In case of `data` can't be deserialized with serde.
#[wasm_bindgen]
pub fn evaluate(data: &JsValue) -> Result<JsValue, JsValue> {
    set_panic_hook_once();

    let data = data.into_serde().map_err(|e| JsValue::from(format!("{}", e)))?;
    let evaluated = temen::evaluate(data).map_err(|e| JsValue::from(format!("{}", e)))?;
    JsValue::from_serde(&evaluated).map_err(|e| JsValue::from(format!("{}", e)))
}
