use yew::prelude::*;
use yew_material_components::{MatSelect, MatListItem, MatButton, WeakComponentLink};
use yew_material_components::list::GraphicType;
use crate::with_raw_code;
use crate::components::Codeblock;

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
        let filled = with_raw_code!(filled { html! {
        <section>
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
        </section>
        }});

        let outlined = with_raw_code!(outlined { html! {
        <section>
            <MatSelect label="Outlined" outlined=true>
                <MatListItem>{""}</MatListItem>
                <MatListItem value="0">{"Option 0"}</MatListItem>
                <MatListItem value="1">{"Option 1"}</MatListItem>
                <MatListItem value="2">{"Option 2"}</MatListItem>
                <MatListItem value="3">{"Option 3"}</MatListItem>
            </MatSelect>
        </section>
        }});

        let preselected = with_raw_code!(preselected { html! {
        <section>
            <MatSelect label="Preselected">
                <MatListItem>{""}</MatListItem>
                <MatListItem value="0">{"Option 0"}</MatListItem>
                <MatListItem value="1">{"Option 1"}</MatListItem>
                <MatListItem value="2" selected=true>{"Option 2"}</MatListItem>
                <MatListItem value="3">{"Option 3"}</MatListItem>
            </MatSelect>
        </section>
        }});

        let has_icon = with_raw_code!(has_icon { html! {
        <section>
            <MatSelect label="Has Icon" outlined=true icon="event">
                <MatListItem>{""}</MatListItem>
                <MatListItem value="0" graphic=GraphicType::Icon>{"Option 0"}</MatListItem>
                <MatListItem value="1" graphic=GraphicType::Icon>{"Option 1"}</MatListItem>
                <MatListItem value="2" graphic=GraphicType::Icon>{"Option 2"}</MatListItem>
                <MatListItem value="3" graphic=GraphicType::Icon>{"Option 3"}</MatListItem>
            </MatSelect>
        </section>
        }});

        let required = with_raw_code!(required { html! {
        <section>
            <MatSelect label="Required filled" required=true>
                <MatListItem>{""}</MatListItem>
                <MatListItem value="0">{"Option 0"}</MatListItem>
                <MatListItem value="1">{"Option 1"}</MatListItem>
                <MatListItem value="2">{"Option 2"}</MatListItem>
                <MatListItem value="3">{"Option 3"}</MatListItem>
            </MatSelect>
        </section>
        }});

        let required_outlined = with_raw_code!(required_outlined { html! {
        <section>
            <MatSelect label="Required outlined" required=true outlined=true>
                <MatListItem>{""}</MatListItem>
                <MatListItem value="0">{"Option 0"}</MatListItem>
                <MatListItem value="1">{"Option 1"}</MatListItem>
                <MatListItem value="2">{"Option 2"}</MatListItem>
                <MatListItem value="3">{"Option 3"}</MatListItem>
            </MatSelect>
        </section>
        }});

        let disabled = with_raw_code!(disabled { html! {
        <section>
            <MatSelect label="Disabled" disabled=true>
                <MatListItem>{""}</MatListItem>
                <MatListItem value="0">{"Option 0"}</MatListItem>
                <MatListItem value="1" disabled=true>{"Option 1"}</MatListItem>
                <MatListItem value="2">{"Option 2"}</MatListItem>
                <MatListItem value="3">{"Option 3"}</MatListItem>
            </MatSelect>
        </section>
        }});

        let natural_width = with_raw_code!(natural_width { html! {
        <section>
            <MatSelect label="Natural width" natural_menu_width=self.natural_menu_width>
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
        }});

        html! {
        <main class="list-demo">

            <Codeblock title="Filled" code_and_html=filled />

            <Codeblock title="Outlined" code_and_html=outlined />

            <Codeblock title="Preselected" code_and_html=preselected />

            <Codeblock title="Has Icon" code_and_html=has_icon />

            <Codeblock title="Required" code_and_html=required />

            <Codeblock title="Required outlined" code_and_html=required_outlined />

            <Codeblock title="Disabled" code_and_html=disabled />

            <Codeblock title="Natural width" code_and_html=natural_width />

        </main>}
    }
}
