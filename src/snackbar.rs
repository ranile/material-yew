use wasm_bindgen::prelude::*;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::{to_option, add_event_listener, set_element_property, add_event_listener_with_callback_to_emit_one_param_to};

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Snackbar;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Snackbar)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Snackbar);

pub struct MatSnackbar {
    props: Props,
    node_ref: NodeRef,
    opening_closure: Option<Closure<dyn FnMut()>>,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closing_closure: Option<Closure<dyn FnMut(JsValue)>>,
    closed_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
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
    #[prop_or_default]
    pub onopening: Callback<()>,
    #[prop_or_default]
    pub onopened: Callback<()>,
    // This is currently a JsValue because I don't want to depend on serde to try and extract the data from it
    // It is up to ths user to do whatever they want from it
    #[prop_or_default]
    pub onclosing: Callback<JsValue>,
    #[prop_or_default]
    pub onclosed: Callback<JsValue>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for MatSnackbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
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
        let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();
        set_element_property(&element, "open", &JsValue::from(self.props.open));
        if first_render {
            let on_opening = self.props.onopening.clone();
            let on_opened = self.props.onopened.clone();

            add_event_listener(&self.node_ref, "MDCSnackbar:opening", move || {
                on_opening.emit(());
            }, &mut self.opening_closure);

            add_event_listener(&self.node_ref, "MDCSnackbar:opened", move || {
                on_opened.emit(());
            }, &mut self.opened_closure);


            add_event_listener_with_callback_to_emit_one_param_to(&self.node_ref, "MDCSnackbar:closing", self.props.onclosing.clone(), &mut self.closing_closure);
            add_event_listener_with_callback_to_emit_one_param_to(&self.node_ref, "MDCSnackbar:closed", self.props.onclosed.clone(), &mut self.closed_closure);
        }
    }
}

