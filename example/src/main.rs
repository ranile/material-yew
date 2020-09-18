use wasm_bindgen::prelude::*;
use exampleapp::App;

fn main() {}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    yew::start_app::<App>();

    Ok(())
}
