use yew::prelude::*;

type TypeaheadController = AttrValue;
#[derive(Properties, PartialEq)]
pub struct Props {
    #[doc = "The ID of the element in the same root node in which the menu should align to. Overrides setting <code>anchorElement = elementReference</code>.<br><strong>NOTE</strong>: anchor or anchorElement must either be an HTMLElement or resolve to an HTMLElement in order for menu to open."]
    #[prop_or(Some(AttrValue::Static("")))]
    pub anchor: Option<AttrValue>,
    #[doc = "Whether the positioning algorithm should calculate relative to the parent of the anchor element (absolute) or relative to the window (fixed).<br>Examples for <code>position = 'fixed'</code>:<br>- If there is no <code>position:relative</code> in the given parent tree and the surface is <code>position:absolute</code> - If the surface is <code>position:fixed</code> - If the surface is in the “top layer” - The anchor and the surface do not share a common <code>position:relative</code> ancestor<br>When using positioning = fixed, in most cases, the menu should position itself above most other <code>position:absolute</code> or <code>position:fixed</code> elements when placed inside of them. e.g. using a menu inside of an <code>md-dialog</code>.<br><strong>NOTE</strong>: Fixed menus will not scroll with the page and will be fixed to the window instead."]
    #[prop_or(Some(AttrValue::Static("absolute")))]
    pub positioning: Option<AttrValue>,
    #[doc = "Skips the opening and closing animations."]
    #[prop_or(Some(false))]
    pub quick: Option<bool>,
    #[doc = "Displays overflow content like a submenu.<br><strong>NOTE</strong>: This may cause adverse effects if you set <code>md-menu {max-height:...}</code> and have items overflowing items in the “y” direction."]
    #[prop_or(Some(false))]
    pub has_overflow: Option<bool>,
    #[doc = "Opens the menu and makes it visible. Alternative to the <code>.show()</code> and <code>.close()</code> methods"]
    #[prop_or(Some(false))]
    pub open: Option<bool>,
    #[doc = "Offsets the menu’s inline alignment from the anchor by the given number in pixels. This value is direction aware and will follow the LTR / RTL direction.<br>e.g. LTR: positive -&gt; right, negative -&gt; left RTL: positive -&gt; left, negative -&gt; right"]
    #[prop_or(Some(0))]
    pub x_offset: Option<usize>,
    #[doc = "Offsets the menu’s block alignment from the anchor by the given number in pixels.<br>e.g. positive -&gt; down, negative -&gt; up"]
    #[prop_or(Some(0))]
    pub y_offset: Option<usize>,
    #[doc = "The max time between the keystrokes of the typeahead menu behavior before it clears the typeahead buffer."]
    #[prop_or(Some(200))]
    pub typepeahead_delay: Option<usize>,
    #[doc = "The corner of the anchor which to align the menu in the standard logical property style of <block>-<inline> e.g. <code>'end-start'</code>.<br>NOTE: This value may not be respected by the menu positioning algorithm if the menu would render outisde the viewport."]
    #[prop_or(Some(AttrValue::Static("end-start")))]
    pub anchor_corner: Option<AttrValue>,
    #[doc = "The corner of the menu which to align the anchor in the standard logical property style of <block>-<inline> e.g. <code>'start-start'</code>.<br>NOTE: This value may not be respected by the menu positioning algorithm if the menu would render outisde the viewport."]
    #[prop_or(Some(AttrValue::Static("start-start")))]
    pub menu_corner: Option<AttrValue>,
    #[doc = "Keeps the user clicks outside the menu.<br>NOTE: clicking outside may still cause focusout to close the menu so see <code>stayOpenOnFocusout</code>."]
    #[prop_or(Some(false))]
    pub stay_open_on_outside_click: Option<bool>,
    #[doc = "Keeps the menu open when focus leaves the menu’s composed subtree.<br>NOTE: Focusout behavior will stop propagation of the focusout event. Set this property to true to opt-out of menu’s focuout handling altogether."]
    #[prop_or(Some(false))]
    pub stay_open_on_focusout: Option<bool>,
    #[doc = "After closing, does not restore focus to the last focused element before the menu was opened."]
    #[prop_or(Some(false))]
    pub skip_restore_focus: Option<bool>,
    #[doc = "The element that should be focused by default once opened.<br>NOTE: When setting default focus to ‘LIST_ROOT’, remember to change <code>tabindex</code> to <code>0</code> and change md-menu’s display to something other than <code>display: contents</code> when necessary."]
    #[prop_or(Some(AttrValue::Static("first-item")))]
    pub default_focus: Option<AttrValue>,
    #[doc = "Whether or not the current menu is a submenu and should not handle specific navigation keys."]
    #[prop_or(Some(false))]
    pub is_submenu: Option<bool>,
    #[doc = "Handles typeahead navigation through the menu."]
    #[prop_or(Some(AttrValue::Static("function { ... }")))]
    pub typepeahead_controller: Option<TypeaheadController>,
    #[doc = ""]
    # [prop_or (Some (wasm_bindgen :: JsValue :: NULL . into ()))]
    pub anchor_element: Option<web_sys::HtmlElement>,
    pub children: Html,
}

#[function_component]
pub fn Menu(props: &Props) -> Html {
    use_effect_with((), |_| {
        crate::import_material_web_module!("/md-web/menu.js")
    });
    html! { <md-menu
       ~anchor={crate::js_value_from_string_or_null(props.anchor.as_ref())}
       ~positioning={crate::js_value_from_string_or_null(props.positioning.as_ref())}
       ~quick={crate::js_value_or_null(props.quick.clone())}
       ~hasOverflow={crate::js_value_or_null(props.has_overflow.clone())}
       open={props.open.unwrap_or_default()}
       ~xOffset={crate::js_value_or_null(props.x_offset.clone())}
       ~yOffset={crate::js_value_or_null(props.y_offset.clone())}
       ~typeaheadDelay={crate::js_value_or_null(props.typepeahead_delay.clone())}
       ~anchorCorner={crate::js_value_from_string_or_null(props.anchor_corner.as_ref())}
       ~menuCorner={crate::js_value_from_string_or_null(props.menu_corner.as_ref())}
       ~stayOpenOnOutsideClick={crate::js_value_or_null(props.stay_open_on_outside_click.clone())}
       ~stayOpenOnFocusout={crate::js_value_or_null(props.stay_open_on_focusout.clone())}
       ~skipRestoreFocus={crate::js_value_or_null(props.skip_restore_focus.clone())}
       ~defaultFocus={crate::js_value_from_string_or_null(props.default_focus.as_ref())}
       ~isSubmenu={crate::js_value_or_null(props.is_submenu.clone())}
       ~typeaheadController={crate::js_value_from_string_or_null(props.typepeahead_controller.as_ref())}
       ~anchorElement={crate::js_value_or_null(props.anchor_element.clone())}>
        {props.children.clone()}
    </md-menu> }
}
