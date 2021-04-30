use crate::components::Codeblock;
use crate::with_raw_code;
use std::borrow::Cow;
use yew::prelude::*;
use yew_material::MatButton;

pub struct Button {}

impl Component for Button {
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
        let standard_button = with_raw_code!(standard_button { html! {
        <section class="demo">
            <MatButton label="Click me!" />
            <MatButton label="Click me!" icon=Cow::from("code") />
        </section>
        }});

        let outlined_button = with_raw_code!(outlined_button { html! {
        <section class="demo">
            <MatButton label="Click me!" outlined=true />
            <MatButton label="Click me!" icon=Cow::from("code") outlined=true />
        </section>
        }});

        let raised_button = with_raw_code!(raised_button { html! {
        <section class="demo">
            <MatButton label="Click me!" raised=true/>
            <MatButton label="Click me!" icon=Cow::from("code") raised=true/>
        </section>
        }});

        let unelevated_button = with_raw_code!(unelevated_button { html! {
        <section class="demo">
            <MatButton label="Click me!" unelevated=true />
            <MatButton label="Click me!" icon=Cow::from("code") unelevated=true />
        </section>
        }});

        let dense_button = with_raw_code!(dense_button { html! {
        <section class="demo">
                <MatButton label="Click me!" dense=true />
                <MatButton label="Click me!" icon=Cow::from("code") dense=true />
            </section>
        }});

        let trailing_icon_button = with_raw_code!(trailing_icon_button { html! {
        <section class="demo">
            <MatButton label="Click me!" icon=Cow::from("code") trailing_icon=true />
            <MatButton label="Click me!" icon=Cow::from("code") outlined=true trailing_icon=true />
            <MatButton label="Click me!" icon=Cow::from("code") raised=true trailing_icon=true />
            <MatButton label="Click me!" icon=Cow::from("code") unelevated=true trailing_icon=true />
            <MatButton label="Click me!" icon=Cow::from("code") dense=true trailing_icon=true />
        </section>
        }});

        let disabled_button = with_raw_code!(disabled_button { html! {
        <section class="demo">
            <MatButton label="Click me!" icon=Cow::from("code") disabled=true />
            <MatButton label="Click me!" icon=Cow::from("code") outlined=true disabled=true />
            <MatButton label="Click me!" icon=Cow::from("code") raised=true disabled=true />
            <MatButton label="Click me!" icon=Cow::from("code") unelevated=true disabled=true />
            <MatButton label="Click me!" icon=Cow::from("code") dense=true disabled=true />
        </section>
        }});

        html! {<>

            <Codeblock title="Standard" code_and_html=standard_button />

            <Codeblock title="Outlined" code_and_html=outlined_button />

            <Codeblock title="Raised" code_and_html=raised_button />

            <Codeblock title="Unelevated" code_and_html=unelevated_button />

            <Codeblock title="Dense" code_and_html=dense_button />

            <Codeblock title="Trailing icon" code_and_html=trailing_icon_button />

            <Codeblock title="Disabled" code_and_html=disabled_button />

        </>}
    }
}
