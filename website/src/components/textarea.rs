use yew::prelude::*;
use yew_material_components::{MatTextArea, ValidityState};
use yew_material_components::textarea::TextAreaCharCounter;

pub struct TextArea {}

impl Component for TextArea {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {


        let tr = MatTextArea::validity_transform(move |_, _| {
            let mut state = ValidityState::new();
            state.set_valid(false)
                .set_bad_input(true);
            state
        });
        html! { <main class="textfield-demo">
            <h2>{"Textarea"}</h2>
            <div class="demo-group-spaced">
                <MatTextArea label="Standard" validity_transform=tr />
                <MatTextArea outlined=true label="Standard" />
            </div>

            <h2>{"Textarea with Character Counter"}</h2>
            <div class="demo-group-spaced">
                <MatTextArea label="Standard" max_length="18" char_counter=TextAreaCharCounter::External />
                <MatTextArea outlined=true label="Standard" max_length="18" char_counter=TextAreaCharCounter::Internal />
            </div>

            <h2>{"Textarea with Helper Text"}</h2>
            <div class="demo-group-spaced shaped-outlined">
                <MatTextArea label="Standard" helper="Helper Text" helper_persistent=true />
                <MatTextArea outlined=true label="Standard" helper="Helper Text" helper_persistent=true />
            </div>
        </main> }
    }
}
