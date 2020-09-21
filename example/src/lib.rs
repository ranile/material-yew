mod progress;
mod button;
mod checkbox;
mod drawer;

use yew::prelude::*;
use crate::drawer::Drawer;
use mat_web_comp::{MatFormfield, MatCheckbox};

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
    <Drawer />
</>

            // <div style="display: flex; flex-direction: column; width: max-content; gap: 1em; padding: 0 1em;">
            // </div>
        }
    }
}
