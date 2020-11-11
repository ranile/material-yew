use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::Node;
use crate::{to_option, add_event_listener, WeakComponentLink};

#[wasm_bindgen(module = "/../build/mwc-dialog.js")]
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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog)
///
/// ## Actions
///
/// In order to pass actions, [`super::MatDialogAction`] component should be used.
pub struct MatDialog {
    props: Props,
    node_ref: NodeRef,
    opening_closure: Option<Closure<dyn FnMut()>>,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closing_closure: Option<Closure<dyn FnMut()>>,
    closed_closure: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {}

/// Props for [`MatDialog`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/dialog#events)
#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub hide_action: bool,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub heading: Option<String>,
    #[prop_or_default]
    pub scrim_click_action: Option<String>,
    #[prop_or_default]
    pub escape_key_action: Option<String>,
    #[prop_or_default]
    pub default_action: Option<String>,
    #[prop_or_default]
    pub action_attribute: Option<String>,
    #[prop_or_default]
    pub initial_focus_attribute: Option<String>,
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
    pub onclosing: Callback<()>,
    /// Binds to `closed` event on `mwc-dialog`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<()>,
    /// [`WeakComponentLink`] for `MatDialog` which provides the following methods
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
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.dialog_link.borrow_mut().replace(link);
        Dialog::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), opened_closure: None, opening_closure: None, closing_closure: None, closed_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
<mwc-dialog
    open=self.props.open
    hideActions?=to_option(self.props.hide_action)
    stacked?=to_option(self.props.stacked)
    heading?=self.props.heading.as_ref()
    scrimClickAction?=self.props.scrim_click_action.as_ref()
    escapeKeyAction?=self.props.escape_key_action.as_ref()
    defaultAction?=self.props.default_action.as_ref()
    actionAttribute?=self.props.action_attribute.as_ref()
    initialFocusAttribute?=self.props.initial_focus_attribute.as_ref()
    ref=self.node_ref.clone()>
    { self.props.children.clone() }
</mwc-dialog>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let onopening = self.props.onopening.clone();
            let onopened = self.props.onopened.clone();
            let onclosing = self.props.onclosing.clone();
            let onclosed = self.props.onclosed.clone();

            add_event_listener(&self.node_ref, "opening", move || {
                onopening.emit(())
            }, &mut self.opening_closure);

            // Doesn't work
            add_event_listener(&self.node_ref, "opened", move || {
                onopened.emit(())
            }, &mut self.opened_closure);

            add_event_listener(&self.node_ref, "closing", move || {
                onclosing.emit(());
            }, &mut self.closing_closure);

            add_event_listener(&self.node_ref, "closed", move || {
                onclosed.emit(());
            }, &mut self.closed_closure);
        }
    }
}

impl WeakComponentLink<MatDialog> {
    pub fn focus(&self) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .focus()
    }

    pub fn blur(&self) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .blur()
    }

    pub fn show(&self) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .show()
    }

    pub fn close(&self) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Dialog>()
            .unwrap()
            .close()
    }
}
