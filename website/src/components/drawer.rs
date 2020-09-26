use yew::prelude::*;

pub struct Drawer {}

impl Component for Drawer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<>
        <section>
            {"Need a way to iframe here. I apparently can't have 2 top app bars"}
        </section>
        </>}
    }
}
