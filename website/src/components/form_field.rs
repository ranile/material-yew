use yew::prelude::*;
use yew_material_components::{MatCheckbox, MatFormfield, MatSwitch, MatRadio};

pub struct FormField {}

impl Component for FormField {
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
                <MatFormfield label="This is a checkbox">
                    <MatCheckbox />
                </MatFormfield>
            </section>

            <section class="comp-demo">
                <h2>{"Align End"}</h2>
                <MatFormfield label="This is another checkbox" align_end=true>
                    <MatCheckbox />
                </MatFormfield>
            </section>

            <section class="comp-demo">
                <h2>{"Switch"}</h2>
                <MatFormfield label="This is a switch">
                    <MatSwitch />
                </MatFormfield>
            </section>

            <section class="comp-demo">
                <h2>{"Radio"}</h2>
                <MatFormfield label="This is a radio button">
                    <MatRadio name="rad" />
                </MatFormfield>
            </section>
        </>}
    }
}
