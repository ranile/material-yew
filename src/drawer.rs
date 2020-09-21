use wasm_bindgen::prelude::*;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::add_event_listener;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type Drawer;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = Drawer)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(Drawer);

pub struct MatDrawer {
    props: Props,
    node_ref: NodeRef,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closed_closure: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub has_header: bool,
    #[prop_or_default]
    pub drawer_type: String,
    #[prop_or_default]
    pub onopened: Callback<()>,
    #[prop_or_default]
    pub onclosed: Callback<()>,
    pub children: Children,
}

impl Component for MatDrawer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Drawer::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), opened_closure: None, closed_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
//         open=self.props.open
<mwc-drawer hasHeader=self.props.has_header ref=self.node_ref.clone()>
    { self.props.children.clone() }
</mwc-drawer>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();

        element.set_attribute("type", &self.props.drawer_type);
        if self.props.open {
            element.set_attribute("open", "true");
        } else {
            element.remove_attribute("open");
        }


        if first_render {
            let onopen_callback = self.props.onopened.clone();
            let onclose_callback = self.props.onclosed.clone();

            add_event_listener(&self.node_ref, "MDCDrawer:opened", move || {
                onopen_callback.emit(());
            }, &mut self.opened_closure);
            add_event_listener(&self.node_ref, "MDCDrawer:closed", move || {
                onclose_callback.emit(());
            }, &mut self.opened_closure);
        }
    }
}
