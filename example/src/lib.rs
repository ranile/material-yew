mod circular_progress;
mod button;

use yew::prelude::*;
use crate::circular_progress::CircularProgress;
use crate::button::Button;

pub struct App {
    closed: bool,
    progress: f32,
    link: ComponentLink<Self>
}

pub enum Msg {
    Close
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {closed: false, link, progress: 0.0}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // self.closed = !self.closed;
        self.progress += 0.1;
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }


    fn view(&self) -> Html {

        html! {
            <div style="display: flex; flex-direction: column; width: max-content; gap: 1em; padding: 0 1em;">
                // <Button />
                <CircularProgress />
            </div>
        }
    }
}
