use wasm_bindgen::JsValue;
use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The anchorCorner to set on the submenu.
    #[prop_or(Some(AttrValue::Static("Corner.START_END")))]
    pub anchor_corner: Option<AttrValue>,
    /// The menuCorner to set on the submenu.
    #[prop_or(Some(AttrValue::Static("Corner.START_START")))]
    pub menu_corner: Option<AttrValue>,
    /// The delay between mouseenter and submenu opening.
    #[prop_or(Some(400))]
    pub hover_open_delay: Option<usize>,
    /// The delay between ponterleave and the submenu closing.
    #[prop_or(Some(400))]
    pub hover_close_delay: Option<usize>,
    /// READONLY: self-identifies as a menu item and sets its identifying attribute
    #[prop_or(Some(true))]
    pub is_sub_menu: Option<bool>,
    ///
    # [prop_or (Some (wasm_bindgen :: JsValue :: NULL . into ()))]
    pub item: Option<JsValue>,
    ///
    # [prop_or (Some (wasm_bindgen :: JsValue :: NULL . into ()))]
    pub menu: Option<JsValue>,
    pub children: Html,
}

#[function_component]
pub fn SubMenu(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/sub-menu.js")
    });
    html! { <md-sub-menu
       ~anchorCorner={crate::js_value_from_string_or_null(props.anchor_corner.as_ref())}
       ~menuCorner={crate::js_value_from_string_or_null(props.menu_corner.as_ref())}
       ~hoverOpenDelay={crate::js_value_or_null(props.hover_open_delay.clone())}
       ~hoverCloseDelay={crate::js_value_or_null(props.hover_close_delay.clone())}
       ~isSubMenu={crate::js_value_or_null(props.is_sub_menu.clone())}
       ~item={crate::js_value_or_null(props.item.clone())}
       ~menu={crate::js_value_or_null(props.menu.clone())}
    >
        {props.children.clone()}
    </md-sub-menu> }
}
