#![cfg(target_family = "wasm")]

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "startRustCal")]
pub async fn start_rustcal() {
    let mut siv = crate::build_app();
    siv.set_fps(30);
    siv.run_with(|| crate::backend::backend()).await;
}
