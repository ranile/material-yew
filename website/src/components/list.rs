use yew::prelude::*;
use yew_material_components::{MatList, MatListItem, MatCheckListItem, MatRadioListItem};

pub struct List {
    link: ComponentLink<Self>,
    basic_selected_index: u32,
    activatable_selected_index: u32,
    checklist_selected_index: u32,
    radio_selected_index: u32,
}

pub enum Msg {
    Action(f64, &'static str),
}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, basic_selected_index: 0, activatable_selected_index: 0, checklist_selected_index: 0, radio_selected_index: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Action(val, which) => {
                match which {
                    "basic" => self.basic_selected_index = val as u32,
                    "activatable" => self.activatable_selected_index = val as u32,
                    "checklist" => self.checklist_selected_index = val as u32,
                    "radio" => self.radio_selected_index = val as u32,
                    _ => panic!("dude you fucked up you should've just used an enum or different messages")
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<div class="list-demo">
            <section>
                <h2>{"Basic"}</h2>
                <MatList onaction=self.link.callback(|val| Msg::Action(val, "basic"))>
                    <MatListItem>{"Item 0"}</MatListItem>
                    <MatListItem>{"Item 1"}</MatListItem>
                    <MatListItem>{"Item 2"}</MatListItem>
                    <MatListItem>{"Item 3"}</MatListItem>
                </MatList>
                <span>{"Selected index: "}{self.basic_selected_index}</span>
            </section>

            <section>
                <h2>{"Multi"}</h2>
                <span>{"Currently unsupported because I got no clue how to expose `number | Set<number>` in Rust"}</span>
            </section>

            <section>
                <h2>{"Activatable"}</h2>
                <MatList activatable=true onaction=self.link.callback(|val| Msg::Action(val, "activatable"))>
                    <MatListItem>{"Item 0"}</MatListItem>
                    <MatListItem>{"Item 1"}</MatListItem>
                    <MatListItem>{"Item 2"}</MatListItem>
                    <MatListItem>{"Item 3"}</MatListItem>
                </MatList>
                <span>{"Selected index: "}{self.activatable_selected_index}</span>
            </section>

            <section>
                <h2>{"Non-interactive"}</h2>
                <MatList noninteractive=true>
                    <MatListItem>{"Item 0"}</MatListItem>
                    <MatListItem>{"Item 1"}</MatListItem>
                    <MatListItem>{"Item 2"}</MatListItem>
                    <MatListItem>{"Item 3"}</MatListItem>
                </MatList>
            </section>

            <section>
                <h2>{"Checklist"}</h2>
                <MatList onaction=self.link.callback(|val| Msg::Action(val, "checklist"))>
                    <MatCheckListItem>{"Item 0"}</MatCheckListItem>
                    <MatCheckListItem>{"Item 1"}</MatCheckListItem>
                    <MatCheckListItem left=true>{"Item 2"}</MatCheckListItem>
                    <MatCheckListItem left=true>{"Item 3"}</MatCheckListItem>
                    // Text needs be in a span to be displayed correctly
                    // Blame Material components
                    <MatCheckListItem disabled=true><span>{"Disabled"}</span></MatCheckListItem>
                </MatList>
                <span>{"Selected index: "}{self.checklist_selected_index}</span>
            </section>

            <section>
                <h2>{"Radio list"}</h2>
                <MatList onaction=self.link.callback(|val| Msg::Action(val, "radio"))>
                    <MatRadioListItem>{"Item 0"}</MatRadioListItem>
                    <MatRadioListItem>{"Item 1"}</MatRadioListItem>
                    <MatRadioListItem left=true>{"Item 2"}</MatRadioListItem>
                    <MatRadioListItem left=true>{"Item 3"}</MatRadioListItem>
                </MatList>
                <span>{"Selected index: "}{self.radio_selected_index}</span>
            </section>
        </div>}
    }
}
