use yew::prelude::*;
use yew_material_components::{MatIconButtonToggle};

pub struct IconButtonToggle {
    link: ComponentLink<Self>,
    state: bool,
}

pub enum Msg {
    Change(bool)
}

impl Component for IconButtonToggle {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, state: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(state) => self.state = state
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<>
            <section class="comp-demo">
                <div>
                    <MatIconButtonToggle on_icon="sentiment_very_satisfied" off_icon="sentiment_very_dissatisfied" onchange=self.link.callback(|state| Msg::Change(state))></MatIconButtonToggle>
                    <br />
                    <span>{"State: "} {self.state}</span>
                </div>

                <MatIconButtonToggle>
                    <svg slot="onIcon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M0 0h24v24H0z" fill="none"/><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
                    <svg slot="offIcon" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path fill="none" d="M0 0h24v24H0V0zm0 0h24v24H0V0z"/><path d="M16.59 7.58L10 14.17l-3.59-3.58L5 12l5 5 8-8zM12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z"/></svg>
                </MatIconButtonToggle>

                <MatIconButtonToggle>
                    <img slot="onIcon" src="https://picsum.photos/id/28/24/24" />
                    <img slot="offIcon" src="https://picsum.photos/id/141/24/24?grayscale" />
                </MatIconButtonToggle>

                <MatIconButtonToggle disabled=true on_icon="sentiment_very_satisfied" off_icon="sentiment_very_dissatisfied"></MatIconButtonToggle>

                <MatIconButtonToggle on_icon="sentiment_very_satisfied" off_icon="sentiment_very_dissatisfied"></MatIconButtonToggle>
            </section>

        </>}
    }
}
