use yew::prelude::*;
use yew_material_components::{MatButton};

pub struct Button {}

impl Component for Button {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<>
            <h2>{"Standard"}</h2>
            <section class="comp-demo">
                <MatButton label="Click me!" />
                <MatButton label="Click me!" icon="code" />
            </section>

            <h2>{"Outlined"}</h2>
            <section class="comp-demo">
                <MatButton label="Click me!" outlined=true />
                <MatButton label="Click me!" icon="code" outlined=true />
            </section>

            <h2>{"Raised"}</h2>
            <section class="comp-demo">
                <MatButton label="Click me!" raised=true/>
                <MatButton label="Click me!" icon="code" raised=true/>
            </section>

            <h2>{"Unelevated"}</h2>
            <section class="comp-demo">
                <MatButton label="Click me!" unelevated=true />
                <MatButton label="Click me!" icon="code" unelevated=true />
            </section>

            <h2>{"Dense"}</h2>
            <section class="comp-demo">
                <MatButton label="Click me!" dense=true />
                <MatButton label="Click me!" icon="code" dense=true />
            </section>

            <h2>{"Trailing icon"}</h2>
            <section class="comp-demo">
                <MatButton label="Click me!" icon="code" trailing_icon=true />
                <MatButton label="Click me!" icon="code" outlined=true trailing_icon=true />
                <MatButton label="Click me!" icon="code" raised=true trailing_icon=true />
                <MatButton label="Click me!" icon="code" unelevated=true trailing_icon=true />
                <MatButton label="Click me!" icon="code" dense=true trailing_icon=true />
            </section>

            <h2>{"Disabled"}</h2>
            <section class="comp-demo">
                <MatButton label="Click me!" icon="code" disabled=true />
                <MatButton label="Click me!" icon="code" outlined=true disabled=true />
                <MatButton label="Click me!" icon="code" raised=true disabled=true />
                <MatButton label="Click me!" icon="code" unelevated=true disabled=true />
                <MatButton label="Click me!" icon="code" dense=true disabled=true />
            </section>
        </>}
    }
}
