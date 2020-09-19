use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::to_option;

#[wasm_bindgen(module = "/build/circular-progress.js")]
extern "C" {
    #[derive(Debug)]
    type CircularProgress;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = CircularProgress)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(CircularProgress);

pub struct MatCircularProgress {
    props: Props
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub indeterminate: bool,
    #[prop_or_default]
    pub progress: f32,
    #[prop_or_default]
    pub density: u32,
    #[prop_or_default]
    pub closed: bool
}

impl Component for MatCircularProgress {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        // call this every time you need the type to be loaded.
        // don't worry, this only runs once

        CircularProgress::ensure_loaded();
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-circular-progress
                indeterminate?=to_option(self.props.indeterminate)
                progress=self.props.progress
                density=self.props.density
                closed?=to_option(self.props.closed)
            ></mwc-circular-progress>
        }
    }
}
