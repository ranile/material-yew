use yew::prelude::*;
use yew_material_components::{MatIcon};

pub struct Icon {}

impl Component for Icon {
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
                <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
                <MatIcon>{"sentiment_dissatisfied"}</MatIcon>
                <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
                <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
                <MatIcon>{"sentiment_very_dissatisfied"}</MatIcon>
            </section>

        </>}
    }
}
