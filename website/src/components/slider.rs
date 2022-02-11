use crate::components::Codeblock;
use crate::with_raw_code;
use material_yew::MatSlider;
use yew::prelude::*;

pub struct Slider {}

impl Component for Slider {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
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
            <Codeblock title="Continuous Slider" code_and_html={continuous} />

            <Codeblock title="Discrete Slider" code_and_html={discrete} />

            /* Undocumented so `disabled` exist
            <section class="comp-demo">
                <h2>{"Disabled"}</h2>
                <MatSlider disabled=true />
            </section>*/
        </main>}
    }
}
