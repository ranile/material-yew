use yew::prelude::*;

pub struct ButtonComponent {}

pub enum Msg {}

impl Component for ButtonComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ButtonComponent {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }


    fn view(&self) -> Html {
        html! {
        <div>
            <mwc-button id="myButton" label="Click Me!" raised="true"></mwc-button>
        </div>
        }
    }
}
