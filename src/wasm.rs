#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "startRustCal")]
pub async fn start_rustcal() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let mut siv = crate::build_app();
    siv.set_fps(30);
    siv.try_run_with(|| crate::backend::backend().map_err(|error| JsValue::from_str(&error.to_string())))
        .await?;

    Ok(())
}
