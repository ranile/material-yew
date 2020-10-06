use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener, to_option, add_event_listener_with_one_param, WeakComponentLink};
use yew::web_sys::Node;
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use js_sys::Object;
use yew::virtual_dom::Attributes::IndexMap;
use crate::list::list_index::ListIndex;
use crate::list::SelectedDetail;

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type List;

    #[wasm_bindgen(getter, static_method_of = List)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn index(this: &List) -> JsValue;

    #[wasm_bindgen(method)]
    fn toggle(this: &List, index: usize, force: bool);

    #[wasm_bindgen(method, js_name = getFocusedItemIndex)]
    fn get_focused_item_index(this: &List) -> usize;

    #[wasm_bindgen(method, js_name = focusItemAtIndex)]
    fn focus_item_at_index(this: &List, index: usize);
}

loader_hack!(List);

pub struct MatList {
    props: Props,
    node_ref: NodeRef,
    action_closure: Option<Closure<dyn FnMut()>>,
    selected_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Properties, Clone)]
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
    pub onaction: Callback<ListIndex>,
    #[prop_or_default]
    pub onselected: Callback<SelectedDetail>,
    #[prop_or_default]
    pub list_link: WeakComponentLink<MatList>,
    pub children: Children,
}

impl Component for MatList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.list_link.borrow_mut().replace(link);
        List::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), action_closure: None, selected_closure: None }
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
            let list = self.node_ref.cast::<List>().unwrap();
            let onaction = self.props.onaction.clone();
            add_event_listener(&self.node_ref, "action", move || {
                let val: JsValue = list.index();

                let index = ListIndex::from(val);
                onaction.emit(index);
            }, &mut self.action_closure);

            let onselected = self.props.onselected.clone();
            add_event_listener_with_one_param(&self.node_ref, "selected", move |value: JsValue| {
                let event = value.unchecked_into::<web_sys::CustomEvent>();
                let val = SelectedDetail::from(event.detail());
                yew::services::ConsoleService::log(&format!("val {:?}", val));
                onselected.emit(val);
            }, &mut self.selected_closure)
        }
    }
}

impl WeakComponentLink<MatList> {
    pub fn toggle(&self, index: usize, force: bool) {
        let list = (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<List>().unwrap();
        list.toggle(index, force)
    }

    pub fn get_focused_item_index(&self) -> usize {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<List>()
            .unwrap()
            .get_focused_item_index()

    }

    pub fn focus_item_at_index(&self, index: usize) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<List>()
            .unwrap()
            .focus_item_at_index(index)
    }
}
