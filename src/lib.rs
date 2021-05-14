// src/lib.rs

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use js_sys::Error as JsError;
use lz_string::{ Decoder };

#[wasm_bindgen]
pub fn decode_base64(source: &str) -> Result<String, JsValue> {
    let decoded = match Decoder::new().decode_base64(source) {
        Ok(string) => string,
        Err(_) => return Err(
            JsError::new("InternalError: Execute lz_string::decode_base64 faild").into()
        )
    };

    Ok(decoded)
}
