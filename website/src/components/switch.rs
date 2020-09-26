use yew::prelude::*;
use yew_material_components::{MatSwitch};

pub struct Switch {}

impl Component for Switch {
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
                <MatSwitch />
            </section>

            <section class="comp-demo">
                <h2>{"Checked"}</h2>
                <MatSwitch checked=true />
            </section>

            <section class="comp-demo">
                <h2>{"Disabled"}</h2>
                <MatSwitch disabled=true />
            </section>

            <section class="comp-demo">
                <h2>{"Disabled checked"}</h2>
                <MatSwitch disabled=true checked=true />
            </section>
        </>}
    }
}
