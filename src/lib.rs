use wasm_bindgen::prelude::*;
use serde_json;
use fingerprint_rs::FingerPrint as ExternalFingerPrint;

mod structs;

use structs::{
    FingerPrint,
    WindowFingerPrint,
    AudioFingerPrint,
    CanvasFingerPrint,
    WebGLFingerPrint,
};

#[wasm_bindgen]
pub async fn get_fingerprint() -> Result<String, JsValue> {
    match ExternalFingerPrint::new().await {
        Some(external_fp) => {
            let my_serializable_fp: FingerPrint = external_fp.into();

            match serde_json::to_string(&my_serializable_fp) {
                Ok(json_string) => Ok(json_string),
                Err(e) => Err(JsValue::from_str(&format!("Erro ao serializar para JSON: {}", e))),
            }
        }
        None => Err(JsValue::from_str("Não foi possível gerar o fingerprint da biblioteca externa")),
    }
}