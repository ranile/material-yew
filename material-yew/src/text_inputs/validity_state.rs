use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub(crate) type ValidityStateJS;

    #[wasm_bindgen(method, setter = badInput)]
    pub fn set_bad_input(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = customError)]
    pub fn set_custom_error(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = patternMismatch)]
    pub fn set_pattern_mismatch(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = rangeOverflow)]
    pub fn set_range_overflow(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = rangeUnderflow)]
    pub fn set_range_underflow(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = too_long)]
    pub fn set_too_long(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = tooShort)]
    pub fn set_too_short(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = type_mismatch)]
    pub fn set_type_mismatch(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = valid)]
    pub fn set_valid(this: &ValidityStateJS, val: bool);

    #[wasm_bindgen(method, setter = valueMissing)]
    pub fn set_value_missing(this: &ValidityStateJS, val: bool);
}

impl Default for ValidityStateJS {
    fn default() -> Self {
        Object::new().unchecked_into()
    }
}

/// Rust type for validity props
pub struct ValidityState {
    bad_input: bool,
    custom_error: bool,
    pattern_mismatch: bool,
    range_overflow: bool,
    range_underflow: bool,
    too_long: bool,
    too_short: bool,
    type_mismatch: bool,
    valid: bool,
    value_missing: bool,
}

impl ValidityState {
    /// Creates a new `ValidityState`.
    ///
    /// All the fields except `valid` is set to false except `valid`, which is
    /// set to `true`
    pub fn new() -> Self {
        Self {
            bad_input: false,
            custom_error: false,
            pattern_mismatch: false,
            range_overflow: false,
            range_underflow: false,
            too_long: false,
            too_short: false,
            type_mismatch: false,
            valid: true,
            value_missing: false,
        }
    }

    pub fn bad_input(&self) -> bool {
        self.bad_input
    }
    pub fn custom_error(&self) -> bool {
        self.custom_error
    }
    pub fn pattern_mismatch(&self) -> bool {
        self.pattern_mismatch
    }
    pub fn range_overflow(&self) -> bool {
        self.range_overflow
    }
    pub fn range_underflow(&self) -> bool {
        self.range_underflow
    }
    pub fn too_long(&self) -> bool {
        self.too_long
    }
    pub fn too_short(&self) -> bool {
        self.too_short
    }
    pub fn type_mismatch(&self) -> bool {
        self.type_mismatch
    }
    pub fn valid(&self) -> bool {
        self.valid
    }
    pub fn value_missing(&self) -> bool {
        self.value_missing
    }

    pub fn set_bad_input(&mut self, value: bool) -> &mut Self {
        self.bad_input = value;
        self
    }
    pub fn set_custom_error(&mut self, value: bool) -> &mut Self {
        self.custom_error = value;
        self
    }
    pub fn set_pattern_mismatch(&mut self, value: bool) -> &mut Self {
        self.pattern_mismatch = value;
        self
    }
    pub fn set_range_overflow(&mut self, value: bool) -> &mut Self {
        self.range_overflow = value;
        self
    }
    pub fn set_range_underflow(&mut self, value: bool) -> &mut Self {
        self.range_underflow = value;
        self
    }
    pub fn set_too_long(&mut self, value: bool) -> &mut Self {
        self.too_long = value;
        self
    }
    pub fn set_too_short(&mut self, value: bool) -> &mut Self {
        self.too_short = value;
        self
    }
    pub fn set_type_mismatch(&mut self, value: bool) -> &mut Self {
        self.type_mismatch = value;
        self
    }
    pub fn set_valid(&mut self, value: bool) -> &mut Self {
        self.valid = value;
        self
    }
    pub fn set_value_missing(&mut self, value: bool) -> &mut Self {
        self.value_missing = value;
        self
    }
}

impl From<ValidityState> for ValidityStateJS {
    fn from(validity_state: ValidityState) -> Self {
        let validity_state_js = ValidityStateJS::default();
        validity_state_js.set_bad_input(validity_state.bad_input());
        validity_state_js.set_custom_error(validity_state.custom_error());
        validity_state_js.set_pattern_mismatch(validity_state.pattern_mismatch());
        validity_state_js.set_range_overflow(validity_state.range_overflow());
        validity_state_js.set_range_underflow(validity_state.range_underflow());
        validity_state_js.set_too_long(validity_state.too_long());
        validity_state_js.set_too_short(validity_state.too_short());
        validity_state_js.set_type_mismatch(validity_state.type_mismatch());
        validity_state_js.set_valid(validity_state.valid());
        validity_state_js.set_value_missing(validity_state.value_missing());
        validity_state_js
    }
}

impl Default for ValidityState {
    fn default() -> Self {
        Self::new()
    }
}
