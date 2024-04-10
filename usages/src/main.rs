#![allow(dead_code)]
use yew::prelude::*;
mod test;

#[function_component]
fn App() -> Html {
    test::test_button()
}

fn main() {
    yew::Renderer::<App>::new().render();
}
