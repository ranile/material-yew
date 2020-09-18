use yew::prelude::*;
use wasm_bindgen::prelude::*;
use mat_web_comp::ButtonComponent;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }


    fn view(&self) -> Html {

        html! {
        <div>
            <p>{ "Hello world!" }</p>
            <ButtonComponent></ButtonComponent>
        </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    yew::start_app::<App>();

    Ok(())
}
