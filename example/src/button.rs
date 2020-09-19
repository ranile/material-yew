use yew::prelude::*;
use mat_web_comp::{MatButton, MatCircularProgress};

pub struct Button {}

pub enum Msg {}

impl Component for Button {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Button {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {
            <div style="display: flex; flex-direction: column; width: max-content; gap: 1em; padding: 0 1em;">
                <MatButton label="Click me!" raised=true icon="code" />
                <MatButton label="Click me!" unelevated=true />
                <MatButton label="Click me!" outlined=true />
                <MatButton label="Click me!" dense=true unelevated=true />
                <MatButton label="Click me!" disabled=true />
                <MatButton label="Click me!" icon="code" trailing_icon=true />
            </div>
        }
    }
}
