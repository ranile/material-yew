use yew::prelude::*;

const SLOT: &str = "title";

/// Props for [`MatDrawerTitle`]
#[derive(Properties, PartialEq, Clone)]
pub struct DrawerTitleProps {
    pub children: Children,
}

/// Defines title for [`MatDrawer`][crate::MatDrawer].
///
/// If the child passed is an element (a `VTag`), then it is modified to include
/// the appropriate attributes. Otherwise, the child is wrapped in a `span`
/// containing said attributes.
pub struct MatDrawerTitle {}

impl Component for MatDrawerTitle {
    type Message = ();
    type Properties = DrawerTitleProps;

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
                        vtag.add_attribute("slot", "title");
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
