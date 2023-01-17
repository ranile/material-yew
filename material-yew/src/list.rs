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

use crate::{bool_to_option, event_into_details, WeakComponentLink};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-list.js")]
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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list)
pub struct MatList {
    node_ref: NodeRef,
    action_listener: Option<EventListener>,
    selected_listener: Option<EventListener>,
}

/// Props for [`MatList`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-1)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#mwc-list-2)
#[derive(Properties, PartialEq, Clone)]
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
    pub item_roles: Option<AttrValue>,
    #[prop_or_default]
    pub inner_role: Option<AttrValue>,
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

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .list_link
            .borrow_mut()
            .replace(ctx.link().clone());
        List::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            action_listener: None,
            selected_listener: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // clear event listeners and update link in case the props changed
        self.action_listener = None;
        self.selected_listener = None;
        ctx.props()
            .list_link
            .borrow_mut()
            .replace(ctx.link().clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-list
                 activatable={bool_to_option(props.activatable) }
                 rootTabbable={bool_to_option(props.root_tabbable)}
                 multi={bool_to_option(props.multi)}
                 wrapFocus={bool_to_option(props.wrap_focus)}
                 itemRoles={props.item_roles.clone()}
                 innerRole={props.inner_role.clone()}
                 noninteractive={bool_to_option(props.noninteractive)}
                 ref={self.node_ref.clone()}
             >
               {props.children.clone()}
             </mwc-list>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let list = self.node_ref.cast::<List>().unwrap();
        if self.selected_listener.is_none() {
            let onselected = props.onselected.clone();
            self.selected_listener = Some(EventListener::new(&list, "selected", move |event| {
                let val = SelectedDetail::from(event_into_details(event));
                onselected.emit(val);
            }));
        }

        if self.action_listener.is_none() {
            let onaction = props.onaction.clone();
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
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#methods) for details
    pub fn toggle(&self, index: usize, force: bool) {
        let list = self
            .borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<List>()
            .unwrap();
        list.toggle(index, force)
    }

    /// Binds to `getFocusedItemIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#methods) for details
    pub fn get_focused_item_index(&self) -> usize {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<List>()
            .unwrap()
            .get_focused_item_index()
    }

    /// Binds to `focusItemAtIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/list#methods) for details
    pub fn focus_item_at_index(&self, index: usize) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<List>()
            .unwrap()
            .focus_item_at_index(index)
    }
}
