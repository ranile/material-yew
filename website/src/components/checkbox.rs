use yew::prelude::*;
use yew_material_components::{MatCheckbox};

pub struct Checkbox {
    link: ComponentLink<Self>,
}

impl Component for Checkbox {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<>
            <section class="comp-demo">
                <h2>{"Standard"}</h2>
                <MatCheckbox />
            </section>

            <section class="comp-demo">
                <h2>{"Checked"}</h2>
                <MatCheckbox checked=true />
            </section>

            <section class="comp-demo">
                <h2>{"Indeterminate"}</h2>
                <MatCheckbox indeterminate=true />
            </section>

            <section class="comp-demo">
                <h2>{"Disabled"}</h2>
                <MatCheckbox disabled=true />
            </section>
        </>}
    }
}
