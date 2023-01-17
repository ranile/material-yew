#[doc(inline)]
pub use crate::list::{ActionDetail, ListIndex, SelectedDetail};

use crate::text_inputs::{
    validity_state::ValidityStateJS, NativeValidityState, ValidityState, ValidityTransform,
};
use crate::utils::WeakComponentLink;
use crate::{bool_to_option, event_into_details, to_option_string};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[wasm_bindgen(module = "/build/mwc-select.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type Select;

    #[wasm_bindgen(getter, static_method_of = Select)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method)]
    fn select(this: &Select, index: usize);

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(
        this: &Select,
        val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>,
    );
}

loader_hack!(Select);

/// The `mwc-select` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/select)
pub struct MatSelect {
    node_ref: NodeRef,
    validity_transform_closure:
        Option<Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>>,
    opened_listener: Option<EventListener>,
    closed_listener: Option<EventListener>,
    action_listener: Option<EventListener>,
    selected_listener: Option<EventListener>,
}

/// Props for [`MatSelect`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/select#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/v0.27.0/packages/select#events)
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: Option<AttrValue>,
    #[prop_or_default]
    pub label: Option<AttrValue>,
    #[prop_or_default]
    pub natural_menu_width: bool,
    #[prop_or_default]
    pub icon: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: Option<AttrValue>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub validation_message: Option<AttrValue>,
    #[prop_or_default]
    pub items: Option<AttrValue>,
    #[prop_or(- 1)]
    pub index: i64,
    #[prop_or_default]
    pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub children: Children,
    /// [`WeakComponentLink`] for `MatList` which provides the following methods
    /// - ```select(&self)```
    ///
    /// See [`WeakComponentLink`] documentation for more information
    #[prop_or_default]
    pub select_link: WeakComponentLink<MatSelect>,
    /// Binds to `opened` event on `mwc-select-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onopened: Callback<()>,
    /// Binds to `closed` event on `mwc-select-surface`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onclosed: Callback<()>,
    /// Binds to `action` event on `mwc-list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onaction: Callback<ActionDetail>,
    /// Binds to `selected` event on `mwc-list`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onselected: Callback<SelectedDetail>,
}

impl Component for MatSelect {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.props()
            .select_link
            .borrow_mut()
            .replace(ctx.link().clone());
        Select::ensure_loaded();
        Self {
            node_ref: NodeRef::default(),
            validity_transform_closure: None,
            opened_listener: None,
            closed_listener: None,
            action_listener: None,
            selected_listener: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
             <mwc-select
                 value={props.value.clone()}
                 label={props.label.clone()}
                 naturalMenuWidth={bool_to_option(props.natural_menu_width)}
                 icon={props.icon.clone()}
                 disabled={props.disabled}
                 outlined={bool_to_option(props.outlined)}
                 helper={props.helper.clone()}
                 required={props.required}
                 validationMessage={props.validation_message.clone()}
                 items={props.items.clone()}
                 index={to_option_string(props.index)}
                 validateOnInitialRender={bool_to_option(props.validate_on_initial_render)}
                 ref={self.node_ref.clone()}
             >
               {props.children.clone()}
             </mwc-select>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // clear event listeners and update link in case the props changed
        self.opened_listener = None;
        self.closed_listener = None;
        self.action_listener = None;
        self.selected_listener = None;
        ctx.props()
            .select_link
            .borrow_mut()
            .replace(ctx.link().clone());
        true
    }

    //noinspection DuplicatedCode
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();
        let element = self.node_ref.cast::<Select>().unwrap();
        if first_render {
            if let Some(transform) = props.validity_transform.clone() {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(
                    move |s: String, v: NativeValidityState| -> ValidityStateJS {
                        transform.0(s, v).into()
                    },
                )
                    as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
                element.set_validity_transform(self.validity_transform_closure.as_ref().unwrap());
            }
        }

        if self.opened_listener.is_none() {
            let onopened = props.onopened.clone();
            self.opened_listener = Some(EventListener::new(&element, "opened", move |_| {
                onopened.emit(())
            }));
        }

        if self.closed_listener.is_none() {
            let onclosed = props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(&element, "closed", move |_| {
                onclosed.emit(())
            }));
        }

        if self.action_listener.is_none() {
            let on_action = props.onaction.clone();
            self.action_listener = Some(EventListener::new(&element, "action", move |event| {
                on_action.emit(ActionDetail::from(event_into_details(event)))
            }));
        }

        if self.selected_listener.is_none() {
            let on_selected = props.onselected.clone();
            self.selected_listener = Some(EventListener::new(&element, "selected", move |event| {
                on_selected.emit(SelectedDetail::from(event_into_details(event)))
            }));
        }
    }
}

impl WeakComponentLink<MatSelect> {
    pub fn select(&self, val: usize) {
        let c = self
            .borrow()
            .as_ref()
            .unwrap()
            .get_component()
            .unwrap()
            .node_ref
            .clone();
        let select_element = c.cast::<Select>().unwrap();
        select_element.select(val);
    }
}

impl MatSelect {
    /// Returns [`ValidityTransform`] to be passed to `validity_transform` prop
    pub fn validity_transform<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform::new(func)
    }
}
