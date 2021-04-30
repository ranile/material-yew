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
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-drawer.js")]
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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer)
pub struct MatDrawer {
    props: DrawerProps,
    node_ref: NodeRef,
    opened_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
}

/// Props for [`MatDrawer`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer#events)
#[derive(Properties, Clone)]
pub struct DrawerProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub has_header: bool,
    #[prop_or_default]
    pub drawer_type: Cow<'static, str>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.drawer_link.borrow_mut().replace(link);
        Drawer::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            opened_listener: None,
            closed_listener: None,
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
        <mwc-drawer hasHeader=bool_to_option(self.props.has_header) ref=self.node_ref.clone()>
            { self.props.children.clone() }
        </mwc-drawer>
                }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<Drawer>().unwrap();
        element.set_type(&JsValue::from(self.props.drawer_type.as_ref()));
        element.set_open(self.props.open);

        if first_render {
            let onopen_callback = self.props.onopened.clone();
            let onclose_callback = self.props.onclosed.clone();

            let element = self.node_ref.cast::<Element>().unwrap();

            self.opened_listener = Some(EventListener::new(
                &element,
                "MDCDrawer:opened",
                move |_| {
                    onopen_callback.emit(());
                },
            ));

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
        let node_ref = (*self.borrow().as_ref().unwrap().get_component().unwrap())
            .node_ref
            .clone();
        let element = node_ref.cast::<Drawer>().unwrap();
        let open = element.open();
        element.set_open(!open);
    }
}
