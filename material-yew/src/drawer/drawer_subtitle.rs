use yew::prelude::*;

const SLOT: &str = "subtitle";

/// Props for [`MatDrawerSubtitle`]
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerSubtitleProps {
    pub children: Children,
}

/// Defines sub title for [`MatDrawer`][crate::MatDrawer].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatDrawerSubtitle {}

impl Component for MatDrawerSubtitle {
    type Message = ();
    type Properties = DrawerSubtitleProps;

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
                        vtag.add_attribute("slot", "subtitle");
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
