use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener, to_option, add_event_listener_with_one_param, WeakComponentLink};
use yew::web_sys::Node;
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use js_sys::Object;
use yew::virtual_dom::Attributes::IndexMap;
use crate::list::{ListIndex, SelectedDetail};
use crate::menu::{Corner, MenuCorner, DefaultFocusState};

#[wasm_bindgen(module = "/build/built-js.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Menu;

    #[wasm_bindgen(getter, static_method_of = Menu)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn index(this: &Menu) -> JsValue;

    #[wasm_bindgen(method)]
    fn show(this: &Menu);

    #[wasm_bindgen(method)]
    fn close(this: &Menu);

    // `MWCMenuIndex` is completely undocumented
    #[wasm_bindgen(method)]
    fn select(this: &Menu, index: &JsValue) -> usize;

    #[wasm_bindgen(method, js_name = getFocusedItemIndex)]
    fn get_focused_item_index(this: &Menu) -> usize;

    #[wasm_bindgen(method, js_name = focusItemAtIndex)]
    fn focus_item_at_index(this: &Menu, index: usize);
}

loader_hack!(Menu);

pub struct MatMenu {
    props: Props,
    node_ref: NodeRef,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closed_closure: Option<Closure<dyn FnMut()>>,
    action_closure: Option<Closure<dyn FnMut()>>,
    selected_closure: Option<Closure<dyn FnMut(JsValue)>>,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub anchor: Option<yew::web_sys::HtmlElement>,
    #[prop_or(Corner::TopStart)]
    pub corner: Corner,
    #[prop_or(MenuCorner::Start)]
    pub menu_corner: MenuCorner,
    #[prop_or_default]
    pub quick: bool,
    #[prop_or_default]
    pub absolute: bool,
    #[prop_or_default]
    pub fixed: bool,
    #[prop_or_default]
    pub x: Option<isize>,
    #[prop_or_default]
    pub y: Option<isize>,
    #[prop_or_default]
    pub force_group_selection: bool,
    #[prop_or(DefaultFocusState::ListRoot)]
    pub default_focus: DefaultFocusState,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub wrap_focus: bool,
    #[prop_or_default]
    pub inner_role: String,
    #[prop_or_default]
    pub multi: bool,
    #[prop_or_default]
    pub activatable: bool,
    #[prop_or_default]
    pub onopened: Callback<()>,
    #[prop_or_default]
    pub onclosed: Callback<()>,
    #[prop_or_default]
    pub onaction: Callback<ListIndex>,
    #[prop_or_default]
    pub onselected: Callback<SelectedDetail>,
    #[prop_or_default]
    pub menu_link: WeakComponentLink<MatMenu>,
    pub children: Children,
}

impl Component for MatMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.menu_link.borrow_mut().replace(link);
        Menu::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), opened_closure: None, closed_closure: None, action_closure: None, selected_closure: None, }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-menu
                open=self.props.open
                corner=self.props.corner.to_string()
                menuCorner=self.props.menu_corner.to_string()
                quick?=to_option(self.props.quick)
                absolute?=to_option(self.props.absolute)
                fixed?=to_option(self.props.fixed)
                x?=self.props.x
                y?=self.props.y
                forceGroupSelection?=to_option(self.props.force_group_selection)
                defaultFocus=self.props.default_focus.to_string()
                fullwidth?=to_option(self.props.fullwidth)
                wrapFocus?=to_option(self.props.wrap_focus)
                innerRole=self.props.inner_role
                multi?=to_option(self.props.multi)
                activatable?=to_option(self.props.activatable)
                ref=self.node_ref.clone()
            >
              { self.props.children.clone() }
            </mwc-menu>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let onopened = self.props.onopened.clone();
            add_event_listener(&self.node_ref, "opened", move || {
                onopened.emit(());
            }, &mut self.opened_closure);

            let onclosed = self.props.onclosed.clone();
            add_event_listener(&self.node_ref, "closed", move || {
                onclosed.emit(());
            }, &mut self.closed_closure);

            let menu = self.node_ref.cast::<Menu>().unwrap();
            let onaction = self.props.onaction.clone();
            add_event_listener(&self.node_ref, "action", move || {
                let val: JsValue = menu.index();

                let index = ListIndex::from(val);
                onaction.emit(index);
            }, &mut self.action_closure);

            let onselected = self.props.onselected.clone();
            add_event_listener_with_one_param(&self.node_ref, "selected", move |value: JsValue| {
                let event = value.unchecked_into::<web_sys::CustomEvent>();
                onselected.emit( SelectedDetail::from(event.detail()));
            }, &mut self.selected_closure)
        }
    }
}

impl WeakComponentLink<MatMenu> {
    pub fn get_focused_item_index(&self) -> usize {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Menu>()
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
            .cast::<Menu>()
            .unwrap()
            .focus_item_at_index(index)
    }

    /// `index` is `JsValue` because `MWCMenuIndex` mentioned in mwc docs is completely undocumented
    pub fn select(&self, index: &JsValue) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .select(index);
    }

    pub fn show(&self) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .show();
    }

    pub fn close(&self) {
        (*self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .close();
    }
}
