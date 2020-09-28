use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::Node;
use crate::{add_event_listener, to_option, to_option_string, NativeValidityState, ValidityState, ValidityTransform};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::utils::WeakComponentLink;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Select;

    #[wasm_bindgen(getter, static_method_of = Select)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method)]
    fn select(this: &Select, index: usize);

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(this: &Select, val: &Closure<dyn FnMut(String, NativeValidityState) -> ValidityState>);
}

loader_hack!(Select);

pub struct MatSelect {
    props: Props,
    node_ref: NodeRef,
    validity_transform_closure: Option<Closure<dyn FnMut(String, NativeValidityState) -> ValidityState>>,
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub natural_menu_width: bool,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub validation_message: String,
    #[prop_or_default]
    pub selected: String,
    #[prop_or_default]
    pub items: String,
    #[prop_or(- 1)]
    pub index: i64,
    #[prop_or_default]
    pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub onaction: Callback<f64>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub select_link: WeakComponentLink<MatSelect>,
}

impl Component for MatSelect {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.select_link.borrow_mut().replace(link);
        Select::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), validity_transform_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-select
                value?=to_option_string(&self.props.value)
                label?=to_option_string(&self.props.label)
                naturalMenuWidth?=to_option(self.props.natural_menu_width)
                icon?=to_option_string(&self.props.icon)
                disabled=self.props.disabled
                outlined?=to_option(self.props.outlined)
                helper?=to_option_string(&self.props.helper)
                required=self.props.required
                validationMessage?=to_option_string(&self.props.validation_message)
                // selected=self.props.selected
                items?=to_option_string(&self.props.items)
                index=self.props.index
                validateOnInitialRender?=to_option(self.props.validate_on_initial_render)
                ref=self.node_ref.clone()
            >
              { self.props.children.clone() }
            </mwc-select>
        }
    }

    //noinspection DuplicatedCode
    fn rendered(&mut self, first_render: bool) {
        if first_render {

            let this = self.node_ref.cast::<Select>().unwrap();

            if let Some(validity_transform) = &self.props.validity_transform {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(*validity_transform.0) as Box<dyn FnMut(String, NativeValidityState) -> ValidityState>));
                this.set_validity_transform(&self.validity_transform_closure.as_ref().unwrap());
            }
        }
    }
}

impl WeakComponentLink<MatSelect> {
    pub fn select(&self, val: usize) {
        let c = (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .clone();
        let select_element = c.cast::<Select>().unwrap();
        select_element.select(val);
    }
}

