use yew::prelude::*;
use yew_material_components::{MatSlider};
use crate::with_raw_code;
use crate::components::Codeblock;

pub struct Slider {}

impl Component for Slider {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        let continuous = with_raw_code!(continuous { html! {
        <section class="demo">
            <MatSlider />
        </section>
        }});

        let discrete = with_raw_code!(discrete { html! {
        <section class="demo">
            <MatSlider max=50 value=10 step=5 />
        </section>
        }});

        html! {<main id="slider-demo">
            <Codeblock title="Continuous Slider" code_and_html=continuous />

            <Codeblock title="Discrete Slider" code_and_html=discrete />

            /* Undocumented so `disabled` exist
            <section class="comp-demo">
                <h2>{"Disabled"}</h2>
                <MatSlider disabled=true />
            </section>*/
        </main>}
    }
}
