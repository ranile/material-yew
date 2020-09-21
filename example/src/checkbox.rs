use yew::prelude::*;
use mat_web_comp::{MatCheckbox, MatButton, MatFormfield};

pub struct Checkbox {
    link: ComponentLink<Self>,
    change: String,
    node_ref: NodeRef
}

pub enum Msg {
    Change(bool),
}

impl Component for Checkbox {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, change: "false".to_string(), node_ref: NodeRef::default() }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::Change(val) => {
                self.change = val.to_string();
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }



    fn view(&self) -> Html {
        html! {
            <div style="display: flex; width: max-content; gap: 1em; padding: 0 1em;">
            {&self.change}
                <MatCheckbox onchange=self.link.callback(|val| Msg::Change(val)) ref=self.node_ref.clone() />
                <MatCheckbox indeterminate=true />
                <MatCheckbox disabled=true />
                <br/>
                <MatFormfield label="Test" align_end=true>
                    <MatCheckbox checked=true />
                </MatFormfield>
            </div>
        }
    }
}
