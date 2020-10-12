use yew::prelude::*;
use crate::{AppRouterAnchor, AppRoute};

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        let getting_started_cargo_code = "
[dependencies]
material-yew-components = { git = \"https://github.com/hamza1311/material-yew-components/\", branch = \"master\" }
        ".trim();

        let getting_started_html_code = "
<link href=\"https://fonts.googleapis.com/css?family=Roboto:300,400,500\" rel=\"stylesheet\">
<link href=\"https://fonts.googleapis.com/css?family=Material+Icons&display=block\" rel=\"stylesheet\">
        ".trim();

        let example_code = "\
use yew_material::MatButton;
use yew::html;

html! {
    <MatButton label=\"Click me!\" />
}
        ".trim();


        html! {
            <section class="home-content">
                <p>
                    {"Yew Material is a components library for "} <a href="https://yew.rs/">{"Yew"}</a> {" framework "}
                    {"which is a wrapper around "} <a href="https://github.com/material-components/material-components-web-components"> {"Material Web Components"} </a> {" exposing Yew components. "}
                    {"All modern browsers are supported. There is no support for polyfills required by Internet Explorer 11"}
                </p>
                <h3>{"Example"}</h3>
                <pre><code>{example_code}</code></pre>

                <h3>{"Getting started"}</h3>
                <h4>{"Installation"}</h4>
                <p>
                    {"Currently this library can only be imported via git. In the future, it'll be available from "} <a href="crates.io">{"crates.io"}</a> {"."}
                </p>
                <span><code>{"Cargo.toml: "}</code></span>
                <pre><code>{getting_started_cargo_code}</code></pre>

                <p>{"Material icons and a Material font can also be imported for full functionality"}</p>
                <span><code>{"index.html: "}</code></span>
                <pre><code>{getting_started_html_code}</code></pre>

                <h3>{"Theming"}</h3>
                <p>
                    {"These components respect the theming applied to Material Web Components using stylesheets."}
                    <a href="https://github.com/material-components/material-components-web-components/blob/master/docs/theming.md">{"Learn about how to theme Material Web Components"}</a>
                </p>

                <h3>{"Documentation"}</h3>
                <p>
                    {"Full API documentation can be found "} <a href="#">{"here."}</a> {" Demos of components can be found "} <AppRouterAnchor route=AppRoute::Components>{"here."}</AppRouterAnchor>
                </p>

                <h3>{"Contributing"}</h3>
                <p>
                    {"If you'd like to help improve these components, just create a "}<a href="https://github.com/hamza1311/yew-material/pulls">{"Pull Request."}</a>
                    {"You can report bugs and issues "}<a href="https://github.com/hamza1311/yew-material/issues">{"here."}</a>
                </p>
            </section>
        }
    }
}
/*

        let getting_started_cargo_code = self.to_highlighted_string("
[dependencies]
material-yew-components = { git = \"https://github.com/hamza1311/material-yew-components/\", branch = \"master\" }
        ".trim(), "rs");

        let getting_started_html_code = self.to_highlighted_string("
<link href=\"https://fonts.googleapis.com/css?family=Roboto:300,400,500\" rel=\"stylesheet\">
<link href=\"https://fonts.googleapis.com/css?family=Material+Icons&display=block\" rel=\"stylesheet\">
        ".trim(), "html");

        let example_code = self.to_highlighted_string("\
use yew_material::MatButton;
use yew::html;

html! {
    <MatButton label=\"Click me!\" />
}
        ".trim(), "rs");

impl Home {
    fn to_highlighted_string(&self, code_block: &str, lang_ext: &str) -> Html {
        let theme = &self.theme_set.themes["base16-mocha.dark"];
        let syntax = self.syntax_set.find_syntax_by_extension(lang_ext).unwrap();

        let highlighed = highlighted_html_for_string(code_block, &self.syntax_set, &syntax, theme);
        Self::html_to_element(&highlighed)
    }

    fn html_to_element(html: &str) -> Html {
        let template: wasm_bindgen::JsValue = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("template")
            .unwrap()
            .into();
        let template: web_sys::HtmlTemplateElement = template.into();
        let html = html.trim();
        template.set_inner_html(html);
        Html::VRef(template.content().first_child().unwrap().into())
    }
}
*/
