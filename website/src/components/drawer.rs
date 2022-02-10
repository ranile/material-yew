use yew::prelude::*;

pub struct Drawer {}

impl Component for Drawer {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {<>
        <section>
            {"Need a way to iframe here. I apparently can't have 2 top app bars"}
        </section>
        </>}
    }
}
