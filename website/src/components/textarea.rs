use crate::components::Codeblock;
use crate::with_raw_code;
use yew::prelude::*;
use yew_material::textarea::TextAreaCharCounter;
use yew_material::{MatTextArea, ValidityState};

pub struct TextArea {}

impl Component for TextArea {
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
        let validity_transform = MatTextArea::validity_transform(move |_, _| {
            let mut state = ValidityState::new();
            state.set_valid(false).set_bad_input(true);
            state
        });

        let standard = with_raw_code!(standard { html! {
        <section class="demo-group-spaced">
            <MatTextArea label="Standard" validity_transform=validity_transform.clone() />
            <MatTextArea outlined=true label="Standard" validity_transform=validity_transform />
            <br />
            <p>{"Note: validation for both of these text areas always fail"}</p>
        </section>
        }});

        let with_char_counter = with_raw_code!(with_char_counter { html! {
        <section class="demo-group-spaced">
            <MatTextArea label="Standard" max_length="18" char_counter=TextAreaCharCounter::External />
            <MatTextArea outlined=true label="Standard" max_length="18" char_counter=TextAreaCharCounter::Internal />
        </section>
        }});

        let with_helper_text = with_raw_code!(with_helper_text { html! {
        <section class="demo-group-spaced shaped-outlined">
            <MatTextArea label="Standard" helper="Helper Text" helper_persistent=true />
            <MatTextArea outlined=true label="Standard" helper="Helper Text" helper_persistent=true />
        </section>
        }});

        html! { <main class="textfield-demo">

            <Codeblock title="Standard Textarea" code_and_html=standard />

            <Codeblock title="Textarea with Character Counter" code_and_html=with_char_counter />

            <Codeblock title="Textarea with Helper Text" code_and_html=with_helper_text />

        </main> }
    }
}
