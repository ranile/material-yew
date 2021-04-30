mod models;
pub use models::*;

use crate::list::{ListIndex, SelectedDetail};
use crate::{bool_to_option, event_into_details, to_option_string, WeakComponentLink};
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-menu.js")]
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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/menu)
pub struct MatMenu {
    props: MenuProps,
    node_ref: NodeRef,
    opened_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
    action_listener: Option<EventListener>,
    selected_listener: Option<EventListener>,
}

/// Props for `MatMenu`
///
/// MWC Documentation [properties](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#propertiesattributes)
/// and [events](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#events)
#[derive(Properties, Clone)]
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
    pub inner_role: Cow<'static, str>,
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
    /// See [`WeakComponentLink`](/yew_material/struct.WeakComponentLink.html)
    /// documentation for more information
    #[prop_or_default]
    pub menu_link: WeakComponentLink<MatMenu>,
    pub children: Children,
}

impl Component for MatMenu {
    type Message = ();
    type Properties = MenuProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.menu_link.borrow_mut().replace(link);
        Menu::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            opened_listener: None,
            closed_listener: None,
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
            <mwc-menu
                open=self.props.open
                corner=to_option_string(self.props.corner.to_string())
                menuCorner=to_option_string(self.props.menu_corner.to_string())
                quick=bool_to_option(self.props.quick)
                absolute=bool_to_option(self.props.absolute)
                fixed=bool_to_option(self.props.fixed)
                x=self.props.x.map(|it| Cow::from(it.to_string()))
                y=self.props.y.map(|it| Cow::from(it.to_string()))
                forceGroupSelection=bool_to_option(self.props.force_group_selection)
                defaultFocus=to_option_string(self.props.default_focus.to_string())
                fullwidth=bool_to_option(self.props.fullwidth)
                wrapFocus=bool_to_option(self.props.wrap_focus)
                innerRole=self.props.inner_role.clone()
                multi=bool_to_option(self.props.multi)
                activatable=bool_to_option(self.props.activatable)
                ref=self.node_ref.clone()
            >
              { self.props.children.clone() }
            </mwc-menu>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let menu = self.node_ref.cast::<Menu>().unwrap();
            if let Some(anchor) = self.props.anchor.as_ref() {
                menu.set_anchor(anchor);
            }
            let onopened = self.props.onopened.clone();
            self.opened_listener = Some(EventListener::new(&menu, "opened", move |_| {
                onopened.emit(());
            }));

            let onclosed = self.props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(&menu, "closed", move |_| {
                onclosed.emit(());
            }));

            let onselected = self.props.onselected.clone();
            self.selected_listener = Some(EventListener::new(&menu, "selected", move |event| {
                onselected.emit(SelectedDetail::from(event_into_details(event)));
            }));

            let onaction = self.props.onaction.clone();
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
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn get_focused_item_index(&self) -> usize {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .get_focused_item_index()
    }

    /// Binds to `focusItemAtIndex` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn focus_item_at_index(&self, index: usize) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
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
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn select(&self, index: &JsValue) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .select(index);
    }

    /// Binds to `show` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn show(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .show();
    }

    /// Binds to `close` method.
    ///
    /// See [here](https://github.com/material-components/material-components-web-components/tree/master/packages/menu#methods) for details
    pub fn close(&self) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .close();
    }

    /// Setter method for `anchor`.
    pub fn set_anchor(&self, anchor: web_sys::HtmlElement) {
        (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .cast::<Menu>()
            .unwrap()
            .set_anchor(&anchor);
    }
}
