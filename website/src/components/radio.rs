use yew::prelude::*;
use yew_material_components::{MatRadio};

pub struct Radio {}

impl Component for Radio {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<>
            <section class="comp-demo">
                <h2>{"Standard"}</h2>
                <MatRadio name="some name" />
            </section>

            <section class="comp-demo">
                <h2>{"Checked"}</h2>
                <MatRadio checked=true />
            </section>

            <section class="comp-demo">
                <h2>{"Disabled"}</h2>
                <MatRadio disabled=true />
            </section>
        </>}
    }
}
