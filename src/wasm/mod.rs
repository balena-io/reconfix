use console_error_panic_hook::set_once as set_panic_hook_once;
use serde_json::{json, Value};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use balena_cdsl::output::generator::Generator;

use crate::{
    schema::Schema,
    validator::{ValidationState, Validator},
};

#[wasm_bindgen]
pub struct JellySchema {
    jelly_schema: Schema,
    json_schema: Value,
    ui_schema: Value,
    last_validation_state: ValidationState,
}

mod temen;

#[wasm_bindgen]
#[allow(non_snake_case)]
impl JellySchema {
    /// Instantiates new JellySchema object
    ///
    /// # Arguments
    ///
    /// * `schema` - JellySchema as a string or an object
    ///
    /// # Throws
    ///
    /// Constructor throws in case of invalid `schema` argument value.
    #[wasm_bindgen(constructor)]
    pub fn constructor(schema: &JsValue) -> Result<JellySchema, JsValue> {
        set_panic_hook_once();

        let jelly_schema: Schema = if schema.is_string() {
            Schema::from_str(&schema.as_string().unwrap()).map_err(|e| JsValue::from(format!("{}", e)))?
        } else {
            schema.into_serde().map_err(|e| JsValue::from(format!("{}", e)))?
        };

        let yaml: serde_yaml::Value = if schema.is_string() {
            serde_yaml::from_str(&schema.as_string().unwrap()).map_err(|e| JsValue::from(format!("{}", e)))?
        } else {
            schema.into_serde().map_err(|e| JsValue::from(format!("{}", e)))?
        };

        let (json_schema, ui_schema) = Generator::with(yaml)
            .map_err(|e| JsValue::from(format!("{:?}", e)))?
            .generate();

        Ok(JellySchema {
            jelly_schema,
            json_schema,
            ui_schema,
            last_validation_state: ValidationState::new(),
        })
    }

    /// Validates data against JellySchema
    ///
    /// # Arguments
    ///
    /// * `data` - A JSON object
    ///
    /// # Throws
    ///
    /// In case of `data` can not be deserialized to a JSON value.
    pub fn validate(&mut self, data: &JsValue) -> Result<bool, JsValue> {
        match data.into_serde() {
            Ok(data) => {
                self.last_validation_state = self.jelly_schema.validate(Some(&data));
                Ok(self.last_validation_state.is_valid())
            }
            Err(e) => {
                self.last_validation_state = ValidationState::new();
                Err(JsValue::from_str(&format!("{}", e)))
            }
        }
    }

    /// Generates JSON Schema & UI Schema object
    ///
    /// ```js
    /// {
    ///     "jsonSchema": {...},
    ///     "uiSchema": {...}
    /// }
    /// ```
    ///
    /// # Throws
    ///
    /// In case of internal error only (serde serialization).
    pub fn jsonAndUiSchema(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&json!({
            "jsonSchema": self.json_schema,
            "uiSchema": self.ui_schema,
        }))
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Returns last validation errors
    ///
    /// # Throws
    ///
    /// In case of internal error only (serde serialization).
    pub fn errors(&self) -> Result<JsValue, JsValue> {
        JsValue::from_serde(&json!(self.last_validation_state.errors()))
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}
