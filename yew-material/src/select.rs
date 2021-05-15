#[doc(inline)]
pub use crate::list::{ActionDetail, ListIndex, SelectedDetail};

use crate::text_inputs::{
    validity_state::ValidityStateJS, NativeValidityState, ValidityState, ValidityTransform,
};
use crate::utils::WeakComponentLink;
use crate::{bool_to_option, event_into_details, to_option_string};
use gloo::events::EventListener;
use std::borrow::Cow;
use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::prelude::*;

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
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/select)
pub struct MatSelect {
    props: Props,
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
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/select#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/select#events)
#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub value: Cow<'static, str>,
    #[prop_or_default]
    pub label: Cow<'static, str>,
    #[prop_or_default]
    pub natural_menu_width: bool,
    #[prop_or_default]
    pub icon: Cow<'static, str>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: Cow<'static, str>,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub validation_message: Cow<'static, str>,
    #[prop_or_default]
    pub items: Cow<'static, str>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.select_link.borrow_mut().replace(link);
        Select::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            validity_transform_closure: None,
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
            <mwc-select
                value=self.props.value.clone()
                label=self.props.label.clone()
                naturalMenuWidth=bool_to_option(self.props.natural_menu_width)
                icon=self.props.icon.clone()
                disabled=self.props.disabled
                outlined=bool_to_option(self.props.outlined)
                helper=self.props.helper.clone()
                required=self.props.required
                validationMessage=self.props.validation_message.clone()
                items=self.props.items.clone()
                index=to_option_string(self.props.index)
                validateOnInitialRender=bool_to_option(self.props.validate_on_initial_render)
                ref=self.node_ref.clone()
            >
              { self.props.children.clone() }
            </mwc-select>
        }
    }

    //noinspection DuplicatedCode
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let element = self.node_ref.cast::<Select>().unwrap();
            if let Some(transform) = self.props.validity_transform.clone() {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(
                    move |s: String, v: NativeValidityState| -> ValidityStateJS {
                        transform.0(s, v).into()
                    },
                )
                    as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
                element.set_validity_transform(&self.validity_transform_closure.as_ref().unwrap());
            }

            let onopened = self.props.onopened.clone();
            self.opened_listener = Some(EventListener::new(&element, "opened", move |_| {
                onopened.emit(())
            }));

            let onclosed = self.props.onclosed.clone();
            self.closed_listener = Some(EventListener::new(&element, "closed", move |_| {
                onclosed.emit(())
            }));

            let on_action = self.props.onaction.clone();
            self.action_listener = Some(EventListener::new(&element, "action", move |event| {
                on_action.emit(ActionDetail::from(event_into_details(event)))
            }));

            let on_selected = self.props.onselected.clone();
            self.selected_listener = Some(EventListener::new(&element, "selected", move |event| {
                on_selected.emit(SelectedDetail::from(event_into_details(event)))
            }));
        }
    }
}

impl WeakComponentLink<MatSelect> {
    pub fn select(&self, val: usize) {
        let c = (*self.borrow().as_ref().unwrap().get_component().unwrap())
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
