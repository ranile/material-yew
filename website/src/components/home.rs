use crate::macros::highlight;
use crate::{html_to_element, AppRoute, AppRouterAnchor};
use yew::prelude::*;

pub struct Home {}

impl Component for Home {
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
        let getting_started_cargo_code = html_to_element(&highlight("
[dependencies]
material-yew-components = { git = \"https://github.com/hamza1311/material-yew-components/\", branch = \"master\", features = [\"full\"] }
        ".trim(), "toml"));

        let example_code = html_to_element(&highlight(
            "\
use yew_material::MatButton;
use yew::html;

html! {
    <MatButton label=\"Click me!\" />
}"
            .trim(),
            "rs",
        ));

        html! {
            <section class="home-content">
                <p>
                    {"Yew Material is a components library for "} <a href="https://yew.rs/">{"Yew"}</a> {" framework "}
                    {"which is a wrapper around "} <a href="https://github.com/material-components/material-components-web-components"> {"Material Web Components"} </a> {" exposing Yew components. "}
                    {"All modern browsers are supported. There is no support for polyfills required by Internet Explorer 11"}
                </p>
                <h2>{"Example"}</h2>
                {example_code}

                <h2>{"Getting started"}</h2>
                <h3>{"Installation"}</h3>
                <p>
                    {"Currently this library can only be imported via git. In the future, it'll be available from "} <a href="crates.io">{"crates.io"}</a> {". Cargo features are used to pick the components to be included."}
                </p>
                {getting_started_cargo_code}

                <p>
                    {"Consider consulting the"} <a href="https://github.com/hamza1311/yew-material#getting-started">{" Github README"}</a> {" for more information about setting up."}
                </p>

                <h2>{"Theming"}</h2>
                <p>
                    {"These components respect the theming applied to Material Web Components using stylesheets. "}
                    <a href="https://github.com/material-components/material-components-web-components/blob/master/docs/theming.md">{"Learn about how to theme Material Web Components"}</a>
                </p>

                <h2>{"Documentation"}</h2>
                <p>
                    {"Full API documentation can be found "} <a href="#">{"here."}</a> {" Demos of components can be found "} <AppRouterAnchor route=AppRoute::Components>{"here."}</AppRouterAnchor>
                </p>

                <h2>{"Contributing"}</h2>
                <p>
                    {"If you'd like to help improve these components, just create a "}<a href="https://github.com/hamza1311/yew-material/pulls">{"Pull Request."}</a>
                    {"You can report bugs and issues "}<a href="https://github.com/hamza1311/yew-material/issues">{"here."}</a>
                </p>
            </section>
        }
    }
}
