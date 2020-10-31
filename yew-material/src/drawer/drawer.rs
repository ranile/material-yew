use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::{add_event_listener};
use web_sys::Node;

#[wasm_bindgen(module = "/../build/mwc-drawer.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Drawer;

    #[wasm_bindgen(getter, static_method_of = Drawer)]
    fn _dummy_loader() -> JsValue;

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
    props: Props,
    node_ref: NodeRef,
    opened_closure: Option<Closure<dyn FnMut()>>,
    closed_closure: Option<Closure<dyn FnMut()>>,
}

pub enum Msg {}

/// Props for [`MatDrawer`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/drawer#events)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub has_header: bool,
    #[prop_or_default]
    pub drawer_type: String,
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
    pub children: Children,
}

impl Component for MatDrawer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Drawer::ensure_loaded();
        Self { props, node_ref: NodeRef::default(), opened_closure: None, closed_closure: None }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
<mwc-drawer hasHeader=self.props.has_header ref=self.node_ref.clone()>
    { self.props.children.clone() }
</mwc-drawer>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<Drawer>().unwrap();
        element.set_type(&JsValue::from(&self.props.drawer_type));
        element.set_open(self.props.open);

        if first_render {
            let onopen_callback = self.props.onopened.clone();
            let onclose_callback = self.props.onclosed.clone();

            add_event_listener(&self.node_ref, "MDCDrawer:opened", move || {
                onopen_callback.emit(());
            }, &mut self.opened_closure);
            add_event_listener(&self.node_ref, "MDCDrawer:closed", move || {
                onclose_callback.emit(());
            }, &mut self.closed_closure);
        }
    }
}
