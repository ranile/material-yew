#![cfg(target_arch = "wasm32")]

use material_yew::MatButton;
use wasm_bindgen_test::*;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
use yew::prelude::*;

#[wasm_bindgen_test]
fn test_button() {

    fn type_of<T>(_: T) -> String {
        let type_name = std::any::type_name::<T>();
        return type_name.to_string();
    }

    let matbutton: Html =  html! {
        <>
          <MatButton label="test" />
        </>
        };

    let minimum_html: Html = html! {
        <>
        </>
    };

    assert_eq!(type_of(&matbutton), type_of(&minimum_html));
}
