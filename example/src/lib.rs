mod progress;
mod button;
mod checkbox;
mod drawer;
mod radio;
mod switch;
mod dialog;

use yew::prelude::*;
use crate::drawer::Drawer;
use crate::dialog::Dialog;

pub struct App {
    link: ComponentLink<Self>,
}

pub enum Msg {
    Close
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }


    fn view(&self) -> Html {

        html! {
<>
    <Dialog />
    // <Drawer />
</>
        }
    }
}
