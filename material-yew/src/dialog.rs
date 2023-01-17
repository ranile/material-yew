mod dialog_action;

pub use dialog_action::*;

use crate::{bool_to_option, event_details_into, WeakComponentLink};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-dialog.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Dialog;

    #[wasm_bindgen(getter, static_method_of = Dialog)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method)]
    fn focus(this: &Dialog);

    #[wasm_bindgen(method)]
    fn blur(this: &Dialog);

    #[wasm_bindgen(method)]
    fn show(this: &Dialog);

    #[wasm_bindgen(method)]
    fn close(this: &Dialog);
}

loader_hack!(Dialog);

/// The `mwc-dialog` component.
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/dialog)
///
/// ## Actions
///
/// In order to pass actions, [`MatDialogAction`] component should be
/// used.
pub struct MatDialog {
    node_ref: NodeRef,
    opening_listener: Option<EventListener>,
    opened_listener: Option<EventListener>,
    closing_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
}

/// Props for [`MatDialog`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/dialog#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/dialog#events)
#[derive(Properties, PartialEq, Clone)]
pub struct DialogProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub hide_action: bool,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub heading: Option<AttrValue>,
    #[prop_or_default]
    pub scrim_click_action: Option<AttrValue>,
    #[prop_or_default]
    pub escape_key_action: Option<AttrValue>,
    #[prop_or_default]
    pub default_action: Option<AttrValue>,
    #[prop_or_default]
    pub action_attribute: Option<AttrValue>,
    #[prop_or_default]
    pub initial_focus_attribute: Option<AttrValue>,
    /// Binds to `opening` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopening: Callback<()>,
    /// Binds to `opened` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closing` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosing: Callback<String>,
    /// Binds to `closed` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<String>,
    /// [`WeakComponentLink`] for `MatDialog` which provides the following
    /// methods:
    /// - ```focus(&self)```
    /// - ```blur(&self)```
    /// - ```show(&self)```
    /// - ```close(&self)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub dialog_link: WeakComponentLink<MatDialog>,
    pub children: Children,
}

impl Component for MatDialog {
    type Message = ();
    type Properties = DialogProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .dialog_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Dialog::ensure_loaded();
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
        <mwc-dialog
            open={props.open}
            hideActions={bool_to_option(props.hide_action)}
            stacked={bool_to_option(props.stacked)}
            heading={props.heading.clone()}
            scrimClickAction={props.scrim_click_action.clone()}
            escapeKeyAction={props.escape_key_action.clone()}
            defaultAction={props.default_action.clone()}
            actionAttribute={props.action_attribute.clone()}
            initialFocusAttribute={props.initial_focus_attribute.clone()}
            ref={self.node_ref.clone()}
            >
            {props.children.clone()}
        </mwc-dialog>
               }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // clear event listeners and update link in case the props changed
        self.opening_listener = None;
        self.opened_listener = None;
        self.closing_listener = None;
        self.closed_listener = None;
        ctx.props()
            .dialog_link
            .borrow_mut()
            .replace(ctx.link().clone());
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<Element>().unwrap();
        if self.opening_listener.is_none() {
            let onopening = props.onopening.clone();
            self.opening_listener = Some(EventListener::new(&element, "opening", move |_| {
                onopening.emit(())
            }));
        }

        if self.opened_listener.is_none() {
            let onopened = props.onopened.clone();
            self.opened_listener = Some(EventListener::new(&element, "opened", move |_| {
                onopened.emit(())
            }));
        }

        if self.closing_listener.is_none() {
            let onclosing = props.onclosing.clone();
            self.closing_listener = Some(EventListener::new(&element, "closing", move |event| {
                onclosing.emit(action_from_event(event))
            }));
        }

        if self.closed_listener.is_none() {
            let onclosed = props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(&element, "closed", move |event| {
                onclosed.emit(action_from_event(event))
            }));
        }
    }
}

impl WeakComponentLink<MatDialog> {
    pub fn focus(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .focus()
    }

    pub fn blur(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .blur()
    }

    pub fn show(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .show()
    }

    pub fn close(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .close()
    }
}

#[wasm_bindgen]
extern "C" {
    type DialogActionType;

    #[wasm_bindgen(method, getter)]
    fn action(this: &DialogActionType) -> String;
}

fn action_from_event(event: &Event) -> String {
    event_details_into::<DialogActionType>(event).action()
}
