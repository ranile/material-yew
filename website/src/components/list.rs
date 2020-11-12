use crate::components::Codeblock;
use crate::with_raw_code;
use yew::prelude::*;
use yew_material::list::ListIndex;
use yew_material::{
    MatButton, MatCheckListItem, MatList, MatListItem, MatRadioListItem, WeakComponentLink,
};

pub struct List {
    link: ComponentLink<Self>,
    list_link: WeakComponentLink<MatList>,
    basic_selected_index: String,
    activatable_selected_index: String,
    checklist_selected_index: String,
    radio_selected_index: String,
    multi_selected_index: String,
}

pub enum Msg {
    Action(ListIndex, &'static str),
    Focus,
}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            basic_selected_index: "".to_string(),
            activatable_selected_index: "".to_string(),
            checklist_selected_index: "".to_string(),
            radio_selected_index: "".to_string(),
            multi_selected_index: "".to_string(),
            list_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let transform = |v| {
            match v {
                ListIndex::Single(o) => {
                    o.map(|it| it.to_string())
                        .unwrap_or_else(|| "none".to_string())
                }
                ListIndex::Multi(s) => {
                    let mut out = String::new();
                    s.iter().for_each(|it| {
                        out.push_str(&it.to_string());
                        out.push_str(", ")
                    });
                    out
                }
            }
        };
        match msg {
            Msg::Action(val, which) => {
                match which {
                    "basic" => self.basic_selected_index = transform(val),
                    "activatable" => self.activatable_selected_index = transform(val),
                    "checklist" => self.checklist_selected_index = transform(val),
                    "radio" => self.radio_selected_index = transform(val),
                    "multi" => self.multi_selected_index = transform(val),
                    _ => {
                        panic!(
                        "dude you fucked up you should've just used an enum or different messages"
                    )
                    }
                }
                true
            }
            Msg::Focus => {
                self.list_link.focus_item_at_index(2);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let basic = with_raw_code!(basic { html! {
        <section>
            <MatList onaction=self.link.callback(|val| Msg::Action(val, "basic")) list_link=self.list_link.clone()>
                <MatListItem>{"Item 0"}</MatListItem>
                <MatListItem>{"Item 1"}</MatListItem>
                <MatListItem>{"Item 2"}</MatListItem>
                <MatListItem>{"Item 3"}</MatListItem>
            </MatList>
            <div>{"Selected index: "}{&self.basic_selected_index}</div>
            <div onclick=self.link.callback(|_| Msg::Focus)>
                <MatButton label="Focus index 2" raised=true />
            </div>
        </section>
        }});

        let multi_activatable = with_raw_code!(multi_activatable { html! {
        <section>
            <MatList onaction=self.link.callback(|val| Msg::Action(val, "multi")) multi=true activatable=true>
                <MatListItem>{"Item 0"}</MatListItem>
                <MatListItem>{"Item 1"}</MatListItem>
                <MatListItem>{"Item 2"}</MatListItem>
                <MatListItem>{"Item 3"}</MatListItem>
            </MatList>

            <span>{"Selected index: "}{&self.multi_selected_index}</span>
        </section>
        }});

        let activatable = with_raw_code!(activatable { html! {
        <section>
            <MatList activatable=true onaction=self.link.callback(|val| Msg::Action(val, "activatable"))>
                <MatListItem>{"Item 0"}</MatListItem>
                <MatListItem>{"Item 1"}</MatListItem>
                <MatListItem>{"Item 2"}</MatListItem>
                <MatListItem>{"Item 3"}</MatListItem>
            </MatList>

            <span>{"Selected index: "}{&self.activatable_selected_index}</span>
        </section>
        }});

        let non_interactive = with_raw_code!(non_interactive { html! {
        <section>
            <MatList noninteractive=true>
                <MatListItem>{"Item 0"}</MatListItem>
                <MatListItem>{"Item 1"}</MatListItem>
                <MatListItem>{"Item 2"}</MatListItem>
                <MatListItem>{"Item 3"}</MatListItem>
            </MatList>
        </section>
        }});

        let checklist = with_raw_code!(checklist { html! {
        <section>
            <MatList onaction=self.link.callback(|val| Msg::Action(val, "checklist"))>
                <MatCheckListItem>{"Item 0"}</MatCheckListItem>
                <MatCheckListItem>{"Item 1"}</MatCheckListItem>
                <MatCheckListItem left=true>{"Item 2"}</MatCheckListItem>
                <MatCheckListItem left=true>{"Item 3"}</MatCheckListItem>
                // Text needs be in a span (or any other element) to be displayed correctly
                // Blame Material components
                <MatCheckListItem disabled=true><span>{"Disabled"}</span></MatCheckListItem>
            </MatList>

            <span>{"Selected index: "}{&self.checklist_selected_index}</span>
        </section>
        }});

        let radio_list = with_raw_code!(radio_list { html! {
        <section>
            <MatList onaction=self.link.callback(|val| Msg::Action(val, "radio"))>
                <MatRadioListItem>{"Item 0"}</MatRadioListItem>
                <MatRadioListItem>{"Item 1"}</MatRadioListItem>
                <MatRadioListItem left=true>{"Item 2"}</MatRadioListItem>
                <MatRadioListItem left=true>{"Item 3"}</MatRadioListItem>
            </MatList>

            <span>{"Selected index: "}{&self.radio_selected_index}</span>
        </section>
        }});

        html! {<main class="list-demo">
            <Codeblock title="Basic" code_and_html=basic />
            <Codeblock title="Multi + Activatable" code_and_html=multi_activatable />
            <Codeblock title="Activatable" code_and_html=activatable />
            <Codeblock title="Non-interactive" code_and_html=non_interactive />
            <Codeblock title="Checklist" code_and_html=checklist />
            <Codeblock title="Radio list" code_and_html=radio_list />
        </main>}
    }
}
