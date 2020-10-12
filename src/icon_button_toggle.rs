use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener, to_option};
use web_sys::Node;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type IconButtonToggle;

    #[wasm_bindgen(getter, static_method_of = IconButtonToggle)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn on(this: &IconButtonToggle) -> bool;
}

loader_hack!(IconButtonToggle);

/// The `mwc-icon-button-toggle` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle)
pub struct MatIconButtonToggle {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>,
}


/// Props for [`MatIconButtonToggle`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/icon-button-toggle#events)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub on: bool,
    #[prop_or_default]
    pub on_icon: String,
    #[prop_or_default]
    pub off_icon: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub disabled: bool,
    /// Binds to `MDCIconButtonToggle:change`.
    ///
    /// Callback's parameter is the `isOn` value passed
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatIconButtonToggle {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        IconButtonToggle::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-icon-button-toggle
                on?=to_option(self.props.on)
                onIcon=self.props.on_icon
                offIcon=self.props.off_icon
                label=self.props.label
                disabled=self.props.disabled
                ref=self.node_ref.clone()
            > { self.props.children.clone() }</mwc-icon-button-toggle>
        }
    }


    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let element = self.node_ref.cast::<IconButtonToggle>().unwrap();

            let callback = self.props.onchange.clone();
            add_event_listener(&self.node_ref, "MDCIconButtonToggle:change", move || {
                callback.emit(element.on())
            }, &mut self.closure);
        }
    }
}
