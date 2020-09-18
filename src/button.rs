// #[cfg(target_feature = "button")]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/build/button.js")]
extern "C" {
    #[derive(Debug)]
    type Button;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Button)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Button);

pub struct MatComponent {
    props: Props
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<String>,
    #[prop_or_default]
    pub raised: bool,
    #[prop_or_default]
    pub unelevated: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub trailing_icon: bool,
}

impl Component for MatComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // call this every time you need the type to be loaded.
        // don't worry, this only runs once
        Button::ensure_loaded();
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <mwc-button
                label=self.props.label
                icon?=self.props.icon.as_ref()
                raised?=to_option(self.props.raised)
                unelevated?=to_option(self.props.unelevated)
                outlined?=to_option(self.props.outlined)
                dense?=to_option(self.props.dense)
                trailingIcon?=to_option(self.props.trailing_icon)
                disabled=self.props.disabled
            ></mwc-button>
        }
    }
}

fn to_option(value: bool) -> Option<&'static str> {
    match value {
        true => Some("true"),
        false => None
    }
}
