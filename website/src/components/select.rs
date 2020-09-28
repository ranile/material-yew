use yew::prelude::*;
use yew_material_components::{MatSelect, MatListItem, MatButton, WeakComponentLink};
use yew_material_components::list::GraphicType;

pub struct Select {
    link: ComponentLink<Self>,
    selected_index: u32,
    natural_menu_width: bool,
    select_link: WeakComponentLink<MatSelect>,
}

pub enum Msg {
    ToggleNaturalMenuWidth,
    SelectIndex2,
}

impl Component for Select {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let select_link = WeakComponentLink::default();
        Self { link, selected_index: 0, natural_menu_width: true, select_link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNaturalMenuWidth => {
                self.natural_menu_width = !self.natural_menu_width;
                true
            }
            Msg::SelectIndex2 => {
                self.select_link.select(2);
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {

        html! {<main class="list-demo">
            <section>
                <h2>{"Filled"}</h2>
                <MatSelect label="Filled" select_link=self.select_link.clone()>
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0">{"Option 0"}</MatListItem>
                    <MatListItem value="1">{"Option 1"}</MatListItem>
                    <MatListItem value="2">{"Option 2"}</MatListItem>
                    <MatListItem value="3">{"Option 3"}</MatListItem>
                </MatSelect>
                <span onclick=self.link.callback(|_| Msg::SelectIndex2)>
                    <MatButton label="Select 'Option 1'"/>
                </span>
                // <span>{"Selected index: "}{self.basic_selected_index}</span>
            </section>

            <section>
                <h2>{"Outlined"}</h2>
                <MatSelect label="Outlined" outlined=true>
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0">{"Option 0"}</MatListItem>
                    <MatListItem value="1">{"Option 1"}</MatListItem>
                    <MatListItem value="2">{"Option 2"}</MatListItem>
                    <MatListItem value="3">{"Option 3"}</MatListItem>
                </MatSelect>
                // <span>{"Selected index: "}{self.basic_selected_index}</span>
            </section>

            <section>
                <h2>{"Preselected"}</h2>
                <MatSelect label="Preselected">
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0">{"Option 0"}</MatListItem>
                    <MatListItem value="1">{"Option 1"}</MatListItem>
                    <MatListItem value="2" selected=true>{"Option 2"}</MatListItem>
                    <MatListItem value="3">{"Option 3"}</MatListItem>
                </MatSelect>
                // <span>{"Selected index: "}{self.basic_selected_index}</span>
            </section>

            <section>
                <h2>{"Has Icon"}</h2>
                <MatSelect label="Has Icon" outlined=true icon="event">
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0" graphic=GraphicType::Icon>{"Option 0"}</MatListItem>
                    <MatListItem value="1" graphic=GraphicType::Icon>{"Option 1"}</MatListItem>
                    <MatListItem value="2" graphic=GraphicType::Icon>{"Option 2"}</MatListItem>
                    <MatListItem value="3" graphic=GraphicType::Icon>{"Option 3"}</MatListItem>
                </MatSelect>
                // <span>{"Selected index: "}{self.basic_selected_index}</span>
            </section>

            <section>
                <h2>{"Required"}</h2>
                <MatSelect label="Required filled" required=true>
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0">{"Option 0"}</MatListItem>
                    <MatListItem value="1">{"Option 1"}</MatListItem>
                    <MatListItem value="2">{"Option 2"}</MatListItem>
                    <MatListItem value="3">{"Option 3"}</MatListItem>
                </MatSelect>
                // <span>{"Selected index: "}{self.basic_selected_index}</span>
            </section>

            <section>
                <h2>{"Required outlined"}</h2>
                <MatSelect label="Required outlined" required=true outlined=true>
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0">{"Option 0"}</MatListItem>
                    <MatListItem value="1">{"Option 1"}</MatListItem>
                    <MatListItem value="2">{"Option 2"}</MatListItem>
                    <MatListItem value="3">{"Option 3"}</MatListItem>
                </MatSelect>
                // <span>{"Selected index: "}{self.basic_selected_index}</span>
            </section>

            <section>
                <h2>{"Disabled"}</h2>
                <MatSelect label="Disabled" disabled=true>
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0">{"Option 0"}</MatListItem>
                    <MatListItem value="1" disabled=true>{"Option 1"}</MatListItem>
                    <MatListItem value="2">{"Option 2"}</MatListItem>
                    <MatListItem value="3">{"Option 3"}</MatListItem>
                </MatSelect>
            </section>

            <section>
                <h2>{"Natural width"}</h2>
                <MatSelect label="Disabled" natural_menu_width=self.natural_menu_width>
                    <MatListItem>{""}</MatListItem>
                    <MatListItem value="0">{"Option 0"}</MatListItem>
                    <MatListItem value="1">{"Option 1"}</MatListItem>
                    <MatListItem value="2">{"Option 2"}</MatListItem>
                    <MatListItem value="3">{"Option 3"}</MatListItem>
                </MatSelect>
                <div onclick=self.link.callback(|_| Msg::ToggleNaturalMenuWidth)>
                    <MatButton label="Toggle natural menu width" raised=true />
                </div>
            </section>
        </main>}
    }
}
