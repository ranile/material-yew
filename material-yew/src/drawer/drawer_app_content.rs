use yew::prelude::*;

const SLOT: &str = "appContent";

/// Props for [`MatDrawerAppContent`]
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerAppContentProps {
    pub children: Children,
}

/// Defines `appContent` for [`MatDrawer`][crate::MatDrawer].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatDrawerAppContent {}

impl Component for MatDrawerAppContent {
    type Message = ();
    type Properties = DrawerAppContentProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let children = props
            .children
            .iter()
            .map(|child| {
                match child {
                    Html::VTag(mut vtag) => {
                        vtag.add_attribute("slot", "appContent");
                        Html::VTag(vtag)
                    }
                    _ => {
                        html! {
                             <span slot={SLOT}>
                                 {child}
                             </span>
                        }
                    }
                }
            })
            .collect::<Html>();

        html! {
             {children}
        }
    }
}
