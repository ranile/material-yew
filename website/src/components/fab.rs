use yew::prelude::*;
use yew_material_components::{MatFab};

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
        html! {<>
            <section class="comp-demo">
                <h2>{"Standard"}</h2>
                <MatFab icon="edit" />
            </section>

            <section class="comp-demo">
                <h2>{"Mini"}</h2>
                <MatFab icon="add" mini=true />
            </section>

            <section class="comp-demo">
                <h2>{"Extended"}</h2>
                <MatFab icon="shopping_cart" label="Add to cart" extended=true />
            </section>
        </>}
    }
}
