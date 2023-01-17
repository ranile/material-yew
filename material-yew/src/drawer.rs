mod drawer_app_content;
mod drawer_header;
mod drawer_subtitle;
mod drawer_title;

pub use drawer_app_content::*;
pub use drawer_header::*;
pub use drawer_subtitle::*;
pub use drawer_title::*;

use crate::{bool_to_option, WeakComponentLink};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-drawer.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Drawer;

    #[wasm_bindgen(getter, static_method_of = Drawer)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn open(this: &Drawer) -> bool;

    #[wasm_bindgen(method, setter)]
    fn set_open(this: &Drawer, value: bool);

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &Drawer, value: &JsValue);
}

loader_hack!(Drawer);

/// The `mwc-drawer` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/drawer)
pub struct MatDrawer {
    node_ref: NodeRef,
    opened_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
}

/// Props for [`MatDrawer`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/drawer#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/drawer#events)
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub has_header: bool,
    #[prop_or_default]
    pub drawer_type: Option<AttrValue>,
    /// Binds to `opened` event on `mwc-drawer`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closed` event on `mwc-drawer`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<()>,
    #[prop_or_default]
    pub drawer_link: WeakComponentLink<MatDrawer>,
    pub children: Children,
}

impl Component for MatDrawer {
    type Message = ();
    type Properties = DrawerProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .drawer_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Drawer::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            opened_listener: None,
            closed_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
        <mwc-drawer hasHeader={bool_to_option(props.has_header)} ref={self.node_ref.clone()}>
            {props.children.clone()}
        </mwc-drawer>
               }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<Drawer>().unwrap();
        element.set_type(&JsValue::from(
            props
                .drawer_type
                .as_ref()
                .map(|s| s.as_ref())
                .unwrap_or_default(),
        ));
        element.set_open(props.open);

        if self.opened_listener.is_none() {
            let onopen_callback = props.onopened.clone();
            self.opened_listener = Some(EventListener::new(
                &element,
                "MDCDrawer:opened",
                move |_| {
                    onopen_callback.emit(());
                },
            ));
        }

        if self.closed_listener.is_none() {
            let onclose_callback = props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(
                &element,
                "MDCDrawer:closed",
                move |_| {
                    onclose_callback.emit(());
                },
            ));
        }
    }
}

impl WeakComponentLink<MatDrawer> {
    /// A convenience method to for `drawer.open = !drawer.open`
    pub fn flip_open_state(&self) {
        let node_ref = self
            .borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .clone();
        let element = node_ref.cast::<Drawer>().unwrap();
        let open = element.open();
        element.set_open(!open);
    }
}
