use crate::components::Codeblock;
use crate::with_raw_code;
use yew::prelude::*;
use yew_material::{
    text_inputs::{TextFieldType, ValidityState},
    MatTextField,
};

pub struct Textfield {}

impl Component for Textfield {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let validity_transform = MatTextField::validity_transform(move |_, _| {
            let mut state = ValidityState::new();
            state.set_valid(false).set_bad_input(true);
            state
        });

        let filled = with_raw_code!(filled { html! {
        <div class="demo-group-spaced">
            <MatTextField label="Standard (always fails validity check)" validity_transform=validity_transform.clone() />
            <MatTextField label="Standard" icon="event" field_type=TextFieldType::Date />
            <MatTextField label="Standard" icon_trailing="delete" />
        </div>
        }});

        let outlined = with_raw_code!(outlined { html! {
        <div class="demo-group-spaced">
            <MatTextField outlined=true label="Outlined (always fails validity check) " validity_transform=validity_transform />
            <MatTextField outlined=true label="Outlined" icon="event" field_type=TextFieldType::Time />
            <MatTextField outlined=true label="Outlined" icon_trailing="delete" />
        </div>
        }});

        let without_label = with_raw_code!(without_label { html! {
        <div class="demo-group-spaced">
            <MatTextField helper="Helper Text" />
            <MatTextField outlined=true helper="Helper Text" />
            <span class="shaped-outlined">
                <MatTextField outlined=true helper="Helper Text" />
            </span>
        </div>
        }});

        let with_char_counter = with_raw_code!(with_char_counter { html! {
        <div class="demo-group-spaced">
            <MatTextField label="Standard" helper="Helper Text" helper_persistent=true max_length="18" char_counter=true />
            <MatTextField outlined=true label="Standard" helper="Helper Text" helper_persistent=true max_length="18" char_counter=true />
            <span class="shaped-outlined">
                <MatTextField outlined=true label="Standard" helper="Helper Text" helper_persistent=true max_length="18" char_counter=true />
            </span>
        </div>
        }});

        html! {
            <main class="textfield-demo">
                <Codeblock title="Filled" code_and_html=filled max_width=100 />

                <Codeblock title="Outlined" code_and_html=outlined max_width=100 />

                <Codeblock title="Text Field without label" code_and_html=without_label max_width=100 />

                <Codeblock title="Text Field with Character Counter" code_and_html=with_char_counter max_width=100 />
            </main>
        }
    }
}
