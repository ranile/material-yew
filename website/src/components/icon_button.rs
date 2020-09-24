use yew::prelude::*;
use yew_material_components::{MatIconButton};

pub struct IconButton {
    link: ComponentLink<Self>,
}

impl Component for IconButton {
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
                <MatIconButton icon="code"></MatIconButton>
            </section>

            <section class="comp-demo">
                <h2>{"SVG"}</h2>
                <MatIconButton>
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"></path><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"></path></svg>
                </MatIconButton>
            </section>

            <section class="comp-demo">
                <h2>{"Disabled"}</h2>
                <MatIconButton disabled=true icon="code"></MatIconButton>
            </section>
        </>}
    }
}
