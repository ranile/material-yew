use wasm_bindgen::prelude::*;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use js_sys::Object;
use crate::{to_option, add_event_listener, add_event_listener_with_one_param, WeakComponentLink};

#[wasm_bindgen(module = "/build/mwc-snackbar.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Snackbar;

    #[wasm_bindgen(getter, static_method_of = Snackbar)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter)]
    fn set_open(this: &Snackbar, value: bool);

    #[wasm_bindgen(method)]
    fn show(this: &Snackbar);

    #[wasm_bindgen(method)]
    fn close(this: &Snackbar, reason: &JsValue);
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    type DetailsReason;

    #[wasm_bindgen(method, getter)]
    fn reason(this: &DetailsReason) -> String;
}

loader_hack!(Snackbar);

/// The `mwc-snackbar` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/snackbar)
pub struct MatSnackbar {
    props: Props,
    node_ref: NodeRef,
    opening_closure: Option<Closure<dyn FnMut()>>,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closing_closure: Option<Closure<dyn FnMut(JsValue)>>,
    closed_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

/// Props for [`MatSnackbar`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/snackbar#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/snackbar#events)
#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub open: bool,
    #[prop_or(5000)]
    pub timeout_ms: i32,
    #[prop_or_default]
    pub close_on_escape: bool,
    #[prop_or_default]
    pub label_text: String,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub leading: bool,
    /// Binds to `MDCSnackbar:opening` event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopening: Callback<()>,
    /// Binds to `MDCSnackbar:opened` event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `MDCSnackbar:` event
    ///
    /// The argument passed to callback corresponds to `reason` parameter of the event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosing: Callback<Option<String>>,
    /// Binds to `closing` event
    ///
    /// The argument passed to callback corresponds to `reason` parameter of the event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<Option<String>>,
    /// [`WeakComponentLink`] for `MatList` which provides the following methods
    /// - ```show(&self)```
    /// - ```close(&self, reason: &str)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub snackbar_link: WeakComponentLink<MatSnackbar>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatSnackbar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.snackbar_link.borrow_mut().replace(link);
        Snackbar::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), opening_closure: None, opened_closure: None, closing_closure: None, closed_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-snackbar
                timeoutMs=self.props.timeout_ms
                closeOnEscape=self.props.close_on_escape
                labelText=self.props.label_text
                stacked?=to_option(self.props.stacked)
                leading?=to_option(self.props.leading)
                ref=self.node_ref.clone()
            >{ self.props.children.clone() }</mwc-snackbar>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<Snackbar>().unwrap();
        element.set_open(self.props.open);

        if first_render {
            let on_opening = self.props.onopening.clone();
            add_event_listener(&self.node_ref, "MDCSnackbar:opening", move || {
                on_opening.emit(());
            }, &mut self.opening_closure);

            let on_opened = self.props.onopened.clone();
            add_event_listener(&self.node_ref, "MDCSnackbar:opened", move || {
                on_opened.emit(());
            }, &mut self.opened_closure);

            let on_closing = self.props.onclosing.clone();
            add_event_listener_with_one_param(&self.node_ref, "MDCSnackbar:closing", move |value| {
                on_closing.emit(js_value_into_details_reason(value));
            }, &mut self.closing_closure);

            let on_closed = self.props.onclosed.clone();
            add_event_listener_with_one_param(&self.node_ref, "MDCSnackbar:closed", move |value| {
                on_closed.emit(js_value_into_details_reason(value));
            }, &mut self.closed_closure);
        }
    }
}

impl WeakComponentLink<MatSnackbar> {
    pub fn show(&self) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Snackbar>()
            .unwrap()
            .show()
    }

    pub fn close(&self, reason: &str) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Snackbar>()
            .unwrap()
            .close(&JsValue::from_str(reason))
    }
}

fn js_value_into_details_reason(value: JsValue) -> Option<String> {
    let event = value.unchecked_into::<web_sys::CustomEvent>();
    let details: JsValue = event.detail();
    if details.is_undefined() {
        None
    } else {
        Some(details.unchecked_into::<DetailsReason>().reason())
    }

}
