use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{to_option, add_event_listener};

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Dialog;

    #[wasm_bindgen(getter, static_method_of = Dialog)]
    fn _dummy_loader() -> JsValue;
}


loader_hack!(Dialog);

// Actions don't work
pub struct MatDialog {
    props: Props,
    node_ref: NodeRef,
    opening_closure: Option<Closure<dyn FnMut()>>,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closing_closure: Option<Closure<dyn FnMut()>>,
    closed_closure: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub hide_action: bool,
    #[prop_or_default]
    pub stacked: bool,
    #[prop_or_default]
    pub heading: String,
    #[prop_or_default]
    pub scrim_click_action: String,
    #[prop_or_default]
    pub escape_key_action: String,
    #[prop_or_default]
    pub default_action: String,
    #[prop_or_default]
    pub action_attribute: String,
    #[prop_or_default]
    pub initial_focus_attribute: String,
    #[prop_or_default]
    pub onopening: Callback<()>,
    #[prop_or_default]
    pub onopened: Callback<()>,
    #[prop_or_default]
    pub onclosing: Callback<()>,
    #[prop_or_default]
    pub onclosed: Callback<()>,
    pub children: Children,
}

impl Component for MatDialog {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
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
    heading=self.props.heading
    scrimClickAction=self.props.scrim_click_action
    escapeKeyAction=self.props.escape_key_action
    defaultAction=self.props.default_action
    actionAttribute=self.props.action_attribute
    initialFocusAttribute=self.props.initial_focus_attribute
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
