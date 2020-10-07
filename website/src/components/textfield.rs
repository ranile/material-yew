use yew::prelude::*;
use yew_material_components::{MatTextField, TextFieldType, ValidityState};

pub struct Textfield {}

impl Component for Textfield {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        let tr = MatTextField::validity_transform(move |_, _| {
            let mut state = ValidityState::new();
            state.set_valid(false)
                .set_bad_input(true);
            state
        });
        html! {<main class="textfield-demo">
            <h2>{"Filled"}</h2>
            <div class="demo-group-spaced">
                <MatTextField label="Standard" validity_transform=tr />
                <MatTextField label="Standard" icon="event" field_type=TextFieldType::Date />
                <MatTextField label="Standard" icon_trailing="delete" />
            </div>

            <h2>{"Outlined"}</h2>
            <div class="demo-group-spaced">
                <MatTextField outlined=true label="Standard" />
                <MatTextField outlined=true label="Standard" icon="event" field_type=TextFieldType::Time />
                <MatTextField outlined=true label="Standard" icon_trailing="delete" />
            </div>

            <h2>{"Shaped Outlined"}</h2>
            <div class="demo-group-spaced shaped-outlined">
                <MatTextField outlined=true label="Email" field_type=TextFieldType::Email />
                <MatTextField outlined=true label="Standard" icon="event" />
                <MatTextField outlined=true label="Standard" icon_trailing="delete" />
            </div>

            <h2>{"Text Field without label"}</h2>
            <div class="demo-group-spaced">
                <MatTextField helper="Helper Text" />
                <MatTextField outlined=true helper="Helper Text" />
                <span class="shaped-outlined">
                    <MatTextField outlined=true helper="Helper Text" />
                </span>
            </div>

            <h2>{"Text Field with Character Counter"}</h2>
            <div class="demo-group-spaced">
                <MatTextField label="Standard" helper="Helper Text" helper_persistent=true max_length="18" char_counter=true />
                <MatTextField outlined=true label="Standard" helper="Helper Text" helper_persistent=true max_length="18" char_counter=true />
                <span class="shaped-outlined">
                    <MatTextField outlined=true label="Standard" helper="Helper Text" helper_persistent=true max_length="18" char_counter=true />
                </span>
            </div>
        </main>}
    }
}
