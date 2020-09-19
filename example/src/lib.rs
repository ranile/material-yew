mod circular_progress;
mod button;
mod checkbox;

use yew::prelude::*;
use circular_progress::CircularProgress;
use button::Button;
use checkbox::Checkbox;

pub struct App {
    link: ComponentLink<Self>
}

pub enum Msg {
    Close
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {link}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }


    fn view(&self) -> Html {

        html! {
            // <div style="display: flex; flex-direction: column; width: max-content; gap: 1em; padding: 0 1em;">
                // <Button />
                // <CircularProgress />
                <Checkbox />
            // </div>
        }
    }
}
