use yew::prelude::*;
use yew_material::{MatFab};
use crate::with_raw_code;
use crate::components::Codeblock;

pub struct Fab {}

impl Component for Fab {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        let standard_fab = with_raw_code!(standard_fab { html! {
        <section class="demo">
            <MatFab icon="edit" />
        </section>
        }});

        let mini_fab = with_raw_code!(mini_fab { html! {
        <section class="demo">
            <MatFab icon="add" mini=true />
        </section>
        }});

        let extended_fab = with_raw_code!(extended_fab { html! {
        <section class="demo">
            <MatFab icon="shopping_cart" label="Add to cart" extended=true />
        </section>
        }});

        html! {<>
            <Codeblock title="Standard" code_and_html=standard_fab />

            <Codeblock title="Mini" code_and_html=mini_fab />

            <Codeblock title="Extended" code_and_html=extended_fab />
        </>}
    }
}
