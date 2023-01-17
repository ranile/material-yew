use crate::{bool_to_option, event_into_details, to_option_string, WeakComponentLink};
use gloo::events::EventListener;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/snackbar)
pub struct MatSnackbar {
    node_ref: NodeRef,
    opening_listener: Option<EventListener>,
    opened_listener: Option<EventListener>,
    closing_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
}

/// Props for [`MatSnackbar`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/snackbar#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/snackbar#events)
#[derive(Properties, PartialEq, Clone)]
pub struct SnackbarProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or(5000)]
    pub timeout_ms: i32,
    #[prop_or_default]
    pub close_on_escape: bool,
    #[prop_or_default]
    pub label_text: Option<AttrValue>,
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
    /// The argument passed to callback corresponds to `reason` parameter of the
    /// event
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosing: Callback<Option<String>>,
    /// Binds to `closing` event
    ///
    /// The argument passed to callback corresponds to `reason` parameter of the
    /// event
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
    type Properties = SnackbarProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .snackbar_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Snackbar::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            opening_listener: None,
            opened_listener: None,
            closing_listener: None,
            closed_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-snackbar
                 timeoutMs={to_option_string(props.timeout_ms)}
                 closeOnEscape={to_option_string(props.close_on_escape)}
                 labelText={props.label_text.clone()}
                 stacked={bool_to_option(props.stacked)}
                 leading={bool_to_option(props.leading)}
                 ref={self.node_ref.clone()}
             >{props.children.clone()}</mwc-snackbar>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<Snackbar>().unwrap();
        element.set_open(props.open);

        if self.opening_listener.is_none() {
            let on_opening = props.onopening.clone();
            self.opening_listener = Some(EventListener::new(
                &element,
                "MDCSnackbar:opening",
                move |_| {
                    on_opening.emit(());
                },
            ));
        };

        if self.opened_listener.is_none() {
            let on_opened = props.onopened.clone();
            self.opened_listener = Some(EventListener::new(
                &element,
                "MDCSnackbar:opened",
                move |_| {
                    on_opened.emit(());
                },
            ));
        };

        if self.closing_listener.is_none() {
            let on_closing = props.onclosing.clone();
            self.closing_listener = Some(EventListener::new(
                &element,
                "MDCSnackbar:closing",
                move |event| {
                    on_closing.emit(event_into_details_reason(event));
                },
            ));
        }

        if self.closed_listener.is_none() {
            let on_closed = props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(
                &element,
                "MDCSnackbar:closed",
                move |event| {
                    on_closed.emit(event_into_details_reason(event));
                },
            ));
        }
    }
}

impl WeakComponentLink<MatSnackbar> {
    pub fn show(&self) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Snackbar>()
            .unwrap()
            .show()
    }

    pub fn close(&self, reason: &str) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Snackbar>()
            .unwrap()
            .close(&JsValue::from_str(reason))
    }
}

fn event_into_details_reason(event: &Event) -> Option<String> {
    let details: JsValue = event_into_details(event);
    if details.is_undefined() {
        None
    } else {
        Some(details.unchecked_into::<DetailsReason>().reason())
    }
}
