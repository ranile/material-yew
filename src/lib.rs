mod button;
pub use button::ButtonComponent;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/button.js")]
extern "C" {
    #[derive(Debug)]
    type Button;
}
