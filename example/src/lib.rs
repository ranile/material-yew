use yew::prelude::*;
use mat_web_comp::{MatComponent};

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
            <div style="display: flex; flex-direction: column; width: max-content; gap: 1em; padding: 0 1em;">
                <MatComponent label="Click me!" raised=true icon="code" />
                <MatComponent label="Click me!" unelevated=true />
                <MatComponent label="Click me!" outlined=true />
                <MatComponent label="Click me!" dense=true unelevated=true />
                <MatComponent label="Click me!" disabled=true />
                <MatComponent label="Click me!" icon="code" trailing_icon=true />
            </div>
        }
    }
}
