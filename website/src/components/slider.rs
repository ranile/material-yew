use yew::prelude::*;
use yew_material_components::{MatSlider};

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
        html! {<>
            <section class="comp-demo">
                <h2>{"Continuous"}</h2>
                <MatSlider />
            </section>

            <section class="comp-demo">
                <h2>{"Discrete"}</h2>
                <MatSlider max=50 value=10 step=5 />
            </section>

            /*<section class="comp-demo">
                <h2>{"Disabled"}</h2>
                <MatSlider disabled=true />
            </section>*/
        </>}
    }
}
