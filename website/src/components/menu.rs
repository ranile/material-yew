use yew::prelude::*;
use yew_material::{MatMenu, WeakComponentLink, MatButton, MatListItem};
use crate::with_raw_code;
use yew_material::menu::{Corner, DefaultFocusState};
use crate::components::Codeblock;

pub struct Menu {
    link: ComponentLink<Self>,
    basic_menu_link: WeakComponentLink<MatMenu>,
    corner_menu_link: WeakComponentLink<MatMenu>,
    quick_menu_link: WeakComponentLink<MatMenu>,
    fixed_menu_link: WeakComponentLink<MatMenu>,
    non_fixed_menu_link: WeakComponentLink<MatMenu>,
    absolute_menu_no_anchor_link: WeakComponentLink<MatMenu>,
    activatable_menu_link: WeakComponentLink<MatMenu>,
    default_focus_menu_link: WeakComponentLink<MatMenu>,
    multi_activatable_menu_link: WeakComponentLink<MatMenu>,
}

pub enum Msg {
    ShowBasicMenu,
    ShowCornerMenu,
    ShowQuickMenu,
    ShowFixedMenu,
    ShowNonFixedMenu,
    ShowAbsoluteMenuNoAnchor,
    ShowActivatableMenu,
    ShowDefaultFoucsMenu,
    ShowMultiActivatableMenu,
}

impl Component for Menu {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            basic_menu_link: WeakComponentLink::default(),
            corner_menu_link: WeakComponentLink::default(),
            quick_menu_link: WeakComponentLink::default(),
            fixed_menu_link: WeakComponentLink::default(),
            non_fixed_menu_link: WeakComponentLink::default(),
            absolute_menu_no_anchor_link: WeakComponentLink::default(),
            activatable_menu_link: WeakComponentLink::default(),
            default_focus_menu_link: WeakComponentLink::default(),
            multi_activatable_menu_link: WeakComponentLink::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ShowBasicMenu => self.basic_menu_link.show(),
            Msg::ShowCornerMenu => self.corner_menu_link.show(),
            Msg::ShowQuickMenu => self.quick_menu_link.show(),
            Msg::ShowFixedMenu => self.fixed_menu_link.show(),
            Msg::ShowNonFixedMenu => self.non_fixed_menu_link.show(),
            Msg::ShowAbsoluteMenuNoAnchor => self.absolute_menu_no_anchor_link.show(),
            Msg::ShowActivatableMenu => self.activatable_menu_link.show(),
            Msg::ShowDefaultFoucsMenu => self.default_focus_menu_link.show(),
            Msg::ShowMultiActivatableMenu => self.multi_activatable_menu_link.show(),
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        let basic_menu = with_raw_code!(basic_menu { html! {
        <div style="position:relative;">
            <span onclick=self.link.callback(|_| Msg::ShowBasicMenu)>
                <MatButton raised=true label="Open Basic Menu"></MatButton>
            </span>
            <MatMenu menu_link=self.basic_menu_link.clone()>
                <MatListItem>{"one"}</MatListItem>
                <MatListItem>{"two"}</MatListItem>
                <MatListItem>{"three"}</MatListItem>
                <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                <li divider=true></li>
                <MatListItem>{"aaa"}</MatListItem>
                <MatListItem>{"bbb"}</MatListItem>
            </MatMenu>
        </div>
        }});

        let corner_menu = with_raw_code!(corner_menu { html! {
        <div style="position:relative;">
            <span onclick=self.link.callback(|_| Msg::ShowCornerMenu)>
                <MatButton
                    raised=true
                    label="Open Menu in BottomRight Corner">
                </MatButton>
            </span>
            // TODO allow user to enter corner value
            <MatMenu corner=Corner::BottomRight menu_link=self.corner_menu_link.clone()>
                <MatListItem>{"one"}</MatListItem>
                <MatListItem>{"two"}</MatListItem>
                <MatListItem>{"three"}</MatListItem>
                <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                <li divider=true></li>
                <MatListItem>{"aaa"}</MatListItem>
                <MatListItem>{"bbb"}</MatListItem>
            </MatMenu>
        </div>
        }});

        let quick_menu = with_raw_code!(quick_menu { html! {
        <div style="position:relative;">
            <span onclick=self.link.callback(|_| Msg::ShowQuickMenu)>
                <MatButton raised=true label="Open Quick Menu"></MatButton>
            </span>
            <MatMenu quick=true menu_link=self.quick_menu_link.clone()>
                <MatListItem>{"one"}</MatListItem>
                <MatListItem>{"two"}</MatListItem>
                <MatListItem>{"three"}</MatListItem>
                <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                <li divider=true></li>
                <MatListItem>{"aaa"}</MatListItem>
                <MatListItem>{"bbb"}</MatListItem>
            </MatMenu>
        </div>
        }});

        let scrollable = with_raw_code!(scrollable_menu { html! {
        <div class="scrollable">
            <span>
                <span onclick=self.link.callback(|_| Msg::ShowFixedMenu)>
                    <MatButton raised=true label="Open Fixed Menu"></MatButton>
                </span>
                <MatMenu fixed=true menu_link=self.fixed_menu_link.clone()>
                    <MatListItem>{"one"}</MatListItem>
                    <MatListItem>{"two"}</MatListItem>
                    <MatListItem>{"three"}</MatListItem>
                    <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                    <li divider=true></li>
                    <MatListItem>{"aaa"}</MatListItem>
                    <MatListItem>{"bbb"}</MatListItem>
                </MatMenu>
            </span>

            <span style="position:relative;">
                <span onclick=self.link.callback(|_| Msg::ShowNonFixedMenu)>
                    <MatButton raised=true label="Open Non-Fixed Menu"></MatButton>
                </span>
                <MatMenu menu_link=self.non_fixed_menu_link.clone()>
                    <MatListItem>{"one"}</MatListItem>
                    <MatListItem>{"two"}</MatListItem>
                    <MatListItem>{"three"}</MatListItem>
                    <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                    <li divider=true></li>
                    <MatListItem>{"aaa"}</MatListItem>
                    <MatListItem>{"bbb"}</MatListItem>
                </MatMenu>
            </span>
            <div>{"Open each menu and then scroll this div"}</div>
            <div class="spacer"></div>
        </div>
        }});

        let absolute_menu_no_anchor = with_raw_code!(absolute_menu_no_anchor { html! {
        <div>
            <span onclick=self.link.callback(|_| Msg::ShowAbsoluteMenuNoAnchor)>
                <MatButton raised=true label="Open Absolute Menu (no anchor)"></MatButton>
            </span>
            <MatMenu
                absolute=true
                x=0
                y=0
                menu_link=self.absolute_menu_no_anchor_link.clone()
            >
                <MatListItem>{"one"}</MatListItem>
                <MatListItem>{"two"}</MatListItem>
                <MatListItem>{"three"}</MatListItem>
                <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                <li divider=true></li>
                <MatListItem>{"aaa"}</MatListItem>
                <MatListItem>{"bbb"}</MatListItem>
            </MatMenu>
        </div>
        }});

        let activatable_menu = with_raw_code!(activatable_menu { html! {
        <div style="position:relative;">
            <span onclick=self.link.callback(|_| Msg::ShowActivatableMenu)>
                <MatButton raised=true label="Open Activatable Menu"></MatButton>
            </span>
            <MatMenu activatable=true menu_link=self.activatable_menu_link.clone()>
                <MatListItem>{"one"}</MatListItem>
                <MatListItem>{"two"}</MatListItem>
                <MatListItem>{"three"}</MatListItem>
                <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                <li divider=true></li>
                <MatListItem>{"aaa"}</MatListItem>
                <MatListItem>{"bbb"}</MatListItem>
            </MatMenu>
        </div>
        }});

        let multi_activatable_menu = with_raw_code!(multi_activatable_menu { html! {
        <div style="position:relative;">
            <span onclick=self.link.callback(|_| Msg::ShowMultiActivatableMenu)>
                <MatButton raised=true label="Open Multi (activatable) Menu"></MatButton>
            </span>
            <MatMenu multi=true activatable=true menu_link=self.multi_activatable_menu_link.clone()>
                <MatListItem selected=true activated=true>{"one"}</MatListItem>
                <MatListItem>{"two"}</MatListItem>
                <MatListItem selected=true activated=true>{"three"}</MatListItem>
                <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                <li divider=true></li>
                <MatListItem>{"aaa"}</MatListItem>
                <MatListItem>{"bbb"}</MatListItem>
            </MatMenu>
        </div>
        }});


        let default_focus_menu = with_raw_code!(default_focus_menu { html! {
        <div style="position:relative;">
            <span onclick=self.link.callback(|_| Msg::ShowDefaultFoucsMenu)>
                <MatButton
                    raised=true
                    label="Open Menu With Default Focus with focus of LastItem">
                </MatButton>
            </span>
            // TODO allow user to enter corner value
            <MatMenu default_focus=DefaultFocusState::LastItem menu_link=self.default_focus_menu_link.clone()>
                <MatListItem>{"one"}</MatListItem>
                <MatListItem>{"two"}</MatListItem>
                <MatListItem>{"three"}</MatListItem>
                <MatListItem disabled=true><div>{"four"}</div></MatListItem>
                <li divider=true></li>
                <MatListItem>{"aaa"}</MatListItem>
                <MatListItem>{"bbb"}</MatListItem>
            </MatMenu>
        </div>
        }});

        html! {
            <main id="menu-demo">
            <Codeblock code_and_html=basic_menu title="Basic Menu" />

            <Codeblock code_and_html=corner_menu title="Corner Menu" />

            <Codeblock code_and_html=quick_menu title="Quick Menu" />

            <Codeblock code_and_html=scrollable title="Scrollable menu" />

            <Codeblock code_and_html=absolute_menu_no_anchor title="Absolute Menu (no anchor)" />

            <Codeblock code_and_html=activatable_menu title="Activatable Menu" />

            <Codeblock code_and_html=multi_activatable_menu title="Multi (activatable) Menu" />

            <Codeblock code_and_html=default_focus_menu title="Menu With Default Focus" />
        </main>
        }
    }
}
