mod list_item;
pub use list_item::*;

mod check_list_item;
pub use check_list_item::*;

mod radio_list_item;
pub use radio_list_item::*;

mod list_index;
pub use list_index::ListIndex;

mod selected_detail;
pub use selected_detail::{IndexDiff, SelectedDetail};

mod action_detail;
pub use action_detail::ActionDetail;

mod request_selected;
pub use request_selected::{RequestSelectedDetail, RequestSelectedSource};

mod graphic_type;
pub use graphic_type::GraphicType;

use crate::{event_details_into, event_into_details, to_option, WeakComponentLink};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-list.js")]
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

/// The `mwc-list` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/list)
pub struct MatList {
    props: ListProps,
    node_ref: NodeRef,
    action_listener: Option<EventListener>,
    selected_listener: Option<EventListener>,
}

/// Props for [`MatList`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-1)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/list#mwc-list-2)
#[derive(Properties, Clone)]
pub struct ListProps {
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
    /// Binds to `action` event on `mwc-list`
    #[prop_or_default]
    pub onaction: Callback<ListIndex>,
    /// Binds to `selected` event `mwc-list`
    #[prop_or_default]
    pub onselected: Callback<SelectedDetail>,
    /// [`WeakComponentLink`] for `MatList` which provides the following methods
    /// - ```toggle(&self, index: usize, force: bool)```
    /// - ```get_focused_item_index(&self) -> usize```
    /// - ```focus_item_at_index(&self, index: usize)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub list_link: WeakComponentLink<MatList>,
    pub children: Children,
}

impl Component for MatList {
    type Message = ();
    type Properties = ListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.list_link.borrow_mut().replace(link);
        List::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            action_listener: None,
            selected_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

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

            let onselected = self.props.onselected.clone();
            self.selected_listener = Some(EventListener::new(&list, "selected", move |event| {
                let val = SelectedDetail::from(event_into_details(event));
                onselected.emit(val);
            }));

            let onaction = self.props.onaction.clone();
            self.action_listener = Some(EventListener::new(&list.clone(), "action", move |_| {
                let val: JsValue = list.index();
                let index = ListIndex::from(val);
                onaction.emit(index);
            }));
        }
    }
}

impl WeakComponentLink<MatList> {
    /// Binds to `toggle` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/list#methods) for details
    pub fn toggle(&self, index: usize, force: bool) {
        let list = (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<List>()
            .unwrap();
        list.toggle(index, force)
    }

    /// Binds to `getFocusedItemIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/list#methods) for details
    pub fn get_focused_item_index(&self) -> usize {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<List>()
            .unwrap()
            .get_focused_item_index()
    }

    /// Binds to `focusItemAtIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/list#methods) for details
    pub fn focus_item_at_index(&self, index: usize) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<List>()
            .unwrap()
            .focus_item_at_index(index)
    }
}
