mod models;

pub use models::*;

use crate::list::{ListIndex, SelectedDetail};
use crate::{bool_to_option, event_into_details, to_option_string, WeakComponentLink};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-menu.js")]
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

    #[wasm_bindgen(method, setter)]
    fn set_anchor(this: &Menu, value: &web_sys::HtmlElement);
}

loader_hack!(Menu);

/// The `mwc-menu` Component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu)
pub struct MatMenu {
    node_ref: NodeRef,
    opened_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
    action_listener: Option<EventListener>,
    selected_listener: Option<EventListener>,
}

/// Props for `MatMenu`
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#events)
#[derive(Properties, PartialEq, Clone)]
pub struct MenuProps {
    /// Changing this prop re-renders the component.
    /// For general usage, consider using `show` method provided by
    /// `WeakComponentLink<MatMenu>` via `menu_link`
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub anchor: Option<web_sys::HtmlElement>,
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
    pub inner_role: Option<AttrValue>,
    #[prop_or_default]
    pub multi: bool,
    #[prop_or_default]
    pub activatable: bool,
    /// Binds to `opened` event on `mwc-menu-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closed` event on `mwc-menu-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<()>,
    /// Binds to `action` event on `mwc-list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onaction: Callback<ListIndex>,
    /// Binds to `selected` event on `mwc-list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onselected: Callback<SelectedDetail>,
    /// `WeakComponentLink` for `MatMenu` which provides the following methods
    /// - `get_focused_item_index(&self) -> usize`
    /// - `focus_item_at_index(&self, index: usize)`
    /// - `select(&self, index: &JsValue)`
    /// - `show(&self)`
    /// - `close(&self)`
    ///
    /// See [`WeakComponentLink`](/material_yew/struct.WeakComponentLink.html)
    /// documentation for more information
    #[prop_or_default]
    pub menu_link: WeakComponentLink<MatMenu>,
    pub children: Children,
}

impl Component for MatMenu {
    type Message = ();
    type Properties = MenuProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .menu_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Menu::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            opened_listener: None,
            closed_listener: None,
            action_listener: None,
            selected_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-menu
                 open={props.open}
                 corner={to_option_string(props.corner.to_string())}
                 menuCorner={to_option_string(props.menu_corner.to_string())}
                 quick={bool_to_option(props.quick)}
                 absolute={bool_to_option(props.absolute)}
                 fixed={bool_to_option(props.fixed)}
                 x={props.x.map(|it| it.to_string())}
                 y={props.y.map(|it| it.to_string())}
                 forceGroupSelection={bool_to_option(props.force_group_selection)}
                 defaultFocus={to_option_string(props.default_focus.to_string())}
                 fullwidth={bool_to_option(props.fullwidth)}
                 wrapFocus={bool_to_option(props.wrap_focus)}
                 innerRole={props.inner_role.clone()}
                 multi={bool_to_option(props.multi)}
                 activatable={bool_to_option(props.activatable)}
                 ref={self.node_ref.clone()}
             >
               {props.children.clone()}
             </mwc-menu>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();
        let menu = self.node_ref.cast::<Menu>().unwrap();
        if first_render {
            if let Some(anchor) = props.anchor.as_ref() {
                menu.set_anchor(anchor);
            }
        }
        if self.opened_listener.is_none() {
            let onopened = props.onopened.clone();
            self.opened_listener = Some(EventListener::new(&menu, "opened", move |_| {
                onopened.emit(());
            }));
        }

        if self.closed_listener.is_none() {
            let onclosed = props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(&menu, "closed", move |_| {
                onclosed.emit(());
            }));
        }

        if self.selected_listener.is_none() {
            let onselected = props.onselected.clone();
            self.selected_listener = Some(EventListener::new(&menu, "selected", move |event| {
                onselected.emit(SelectedDetail::from(event_into_details(event)));
            }));
        }

        if self.action_listener.is_none() {
            let onaction = props.onaction.clone();
            self.action_listener = Some(EventListener::new(&menu.clone(), "action", move |_| {
                let val: JsValue = menu.index();

                let index = ListIndex::from(val);
                onaction.emit(index);
            }));
        }
    }
}

impl WeakComponentLink<MatMenu> {
    /// Binds to `getFocusedItemIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#methods) for details
    pub fn get_focused_item_index(&self) -> usize {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .get_focused_item_index()
    }

    /// Binds to `focusItemAtIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#methods) for details
    pub fn focus_item_at_index(&self, index: usize) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .focus_item_at_index(index)
    }

    /// Binds to `select` method.
    ///
    /// `index` is `JsValue` because `MWCMenuIndex` mentioned in mwc docs is
    /// completely undocumented.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#methods) for details
    pub fn select(&self, index: &JsValue) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .select(index);
    }

    /// Binds to `show` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#methods) for details
    pub fn show(&self) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .show();
    }

    /// Binds to `close` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/menu#methods) for details
    pub fn close(&self) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .close();
    }

    /// Setter method for `anchor`.
    pub fn set_anchor(&self, anchor: web_sys::HtmlElement) {
        self.borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .set_anchor(&anchor);
    }
}
