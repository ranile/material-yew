use wasm_bindgen::prelude::*;
use js_sys::Object;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type ValidityState;

    #[wasm_bindgen(method, setter = badInput)]
    pub fn set_bad_input(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = customError)]
    pub fn set_custom_error(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = patternMismatch)]
    pub fn set_pattern_mismatch(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = rangeOverflow)]
    pub fn set_range_overflow(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = rangeUnderflow)]
    pub fn set_range_underflow(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = too_long)]
    pub fn set_too_long(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = tooShort)]
    pub fn set_too_short(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = type_mismatch)]
    pub fn set_type_mismatch(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = valid)]
    pub fn set_valid(this: &ValidityState, val: bool);

    #[wasm_bindgen(method, setter = valueMissing)]
    pub fn set_value_missing(this: &ValidityState, val: bool);
}


impl Default for ValidityState {
    fn default() -> Self {
        Object::new().unchecked_into()
    }
}
