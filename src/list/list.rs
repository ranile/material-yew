use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener, to_option};

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    type List;

    // This needs to be added to each component
    #[wasm_bindgen(getter, static_method_of = List)]
    fn _dummy_loader() -> JsValue;
}

// call the macro with the type
loader_hack!(List);

pub struct MatList {
    props: Props,
    node_ref: NodeRef,
    action_closure: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {}

#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub activatable: bool,
    #[prop_or_default]
    pub root_tabbable: bool,
    #[prop_or_default]
    pub multi: bool,
    #[prop_or_default]
    pub wrap_focus: bool,
    #[prop_or_default]
    pub item_roles: Option<String>,
    #[prop_or_default]
    pub inner_role: Option<String>,
    #[prop_or_default]
    pub noninteractive: bool,
    #[prop_or_default]
    pub onaction: Callback<f64>,
    pub children: Children,
}

impl Component for MatList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        List::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), action_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-list
                activatable?=to_option(self.props.activatable)
                rootTabbable?=to_option(self.props.root_tabbable)
                multi?=to_option(self.props.multi)
                wrapFocus?=to_option(self.props.wrap_focus)
                itemRoles=self.props.item_roles.as_ref().unwrap_or(&"null".to_string())
                innerRole=self.props.inner_role.as_ref().unwrap_or(&"null".to_string())
                noninteractive?=to_option(self.props.noninteractive)
                ref=self.node_ref.clone()
            >
              { self.props.children.clone() }
            </mwc-list>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let element = self.node_ref.cast::<yew::web_sys::Element>().unwrap();
            let onaction = self.props.onaction.clone();
            add_event_listener(&self.node_ref, "action", move|| {
                let index = js_sys::Reflect::get(&element, &JsValue::from_str("index"))
                    .unwrap()
                    .as_f64()
                    .unwrap();
                yew::services::ConsoleService::log(&format!("index {}", index));
                onaction.emit(index);

            }, &mut self.action_closure)

            // selected
        }
    }
}
