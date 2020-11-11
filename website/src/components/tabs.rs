use crate::components::Codeblock;
use crate::with_raw_code;
use yew::prelude::*;
use yew_material::{tabs::MatTabIcon, MatTab, MatTabBar};

pub struct Tabs {}

impl Component for Tabs {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let standard = with_raw_code!(standard { html! {
        <MatTabBar>
            <MatTab label="one" />
            <MatTab label="two" />
            <MatTab label="three" />
        </MatTabBar>
        }});

        let fading_indicator = with_raw_code!(fading_indicator { html! {
        <section>
            <MatTabBar>
                <MatTab is_fading_indicator=true label="one" />
                <MatTab is_fading_indicator=true label="two" />
                <MatTab is_fading_indicator=true label="three" />
            </MatTabBar>
        </section>
        }});

        let min_width_tab = with_raw_code!(min_width_tab { html! {
        <section>
            <MatTabBar>
                <MatTab min_width=true label="one" />
                <MatTab min_width=true label="two" />
                <MatTab min_width=true label="three" />
            </MatTabBar>
        </section>
        }});

        let min_width_indicator = with_raw_code!(min_width_indicator { html! {
        <section>
            <MatTabBar>
                <MatTab is_min_width_indicator=true label="one" />
                <MatTab is_min_width_indicator=true label="two" />
                <MatTab is_min_width_indicator=true label="three" />
            </MatTabBar>
        </section>
        }});

        let min_width_tab_indicator = with_raw_code!(min_width_tab_indicator { html! {
        <section>
            <MatTabBar>
                <MatTab is_min_width_indicator=true min_width=true label="one" />
                <MatTab is_min_width_indicator=true min_width=true label="two" />
                <MatTab is_min_width_indicator=true min_width=true label="three" />
            </MatTabBar>
        </section>
        }});

        let with_icons = with_raw_code!(with_icons { html! {
        <section>
            <MatTabBar>
                <MatTab icon="accessibility" label="one" />
                <MatTab icon="exit_to_app" label="two" />
                <MatTab icon="camera" label="three" />
            </MatTabBar>
        </section>
        }});

        let only_icons = with_raw_code!(only_icons { html! {
        <section>
            <MatTabBar>
                <MatTab icon="accessibility" />
                <MatTab icon="exit_to_app" />
                <MatTab icon="camera" />
            </MatTabBar>
        </section>
        }});

        let image_icons = with_raw_code!(image_icons { html! {
        <section>
            <MatTabBar>
                <MatTab>
                    <MatTabIcon>
                        <svg width="10px" height="10px"><circle r="5px" cx="5px" cy="5px" color="red"></circle></svg>
                    </MatTabIcon>
                </MatTab>
                <MatTab>
                    <MatTabIcon>
                        <svg width="10px" height="10px"><rect width="10px" height="10px" color="green"></rect></svg>
                    </MatTabIcon>
                </MatTab>
                <MatTab>
                    <MatTabIcon>
                        <svg width="10px" height="10px"><rect width="10px" height="10px" color="green"></rect></svg>
                    </MatTabIcon>
                </MatTab>
            </MatTabBar>
        </section>
        }});

        let with_icons_stacked = with_raw_code!(with_icons_stacked { html! {
        <section>
            <MatTabBar>
                <MatTab icon="accessibility" label="one" stacked=true />
                <MatTab icon="exit_to_app" label="two" stacked=true />
                <MatTab icon="camera" label="three" stacked=true />
            </MatTabBar>
        </section>
        }});

        let with_icons_stacked_min_width = with_raw_code!(with_icons_stacked_min_width { html! {
        <section>
            <MatTabBar>
                <MatTab icon="accessibility" label="one" stacked=true is_min_width_indicator=true />
                <MatTab icon="exit_to_app" label="two" stacked=true is_min_width_indicator=true />
                <MatTab icon="camera" label="three" stacked=true is_min_width_indicator=true />
            </MatTabBar>
        </section>
        }});

        let scrolling = with_raw_code!(scrolling { html! {
        <section style="width: 390px">
            <MatTabBar>
                <MatTab label="one" />
                <MatTab label="two" />
                <MatTab label="three" />
                <MatTab label="four" />
                <MatTab label="five" />
                <MatTab label="six" />
                <MatTab label="seven" />
                <MatTab label="eight" />
                <MatTab label="nine" />
                <MatTab label="ten" />
            </MatTabBar>
        </section>
        }});

        let scrolling_with_icons_stacked_min_width = with_raw_code!(scrolling_with_icons_stacked_min_width { html! {
        <section style="width: 460px">
            <MatTabBar>
                <MatTab label="one" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="two" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="three" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="four" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="five" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="six" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="seven" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="eight" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="nine" stacked=true is_min_width_indicator=true icon="camera" />
                <MatTab label="ten" stacked=true is_min_width_indicator=true icon="camera" />
            </MatTabBar>
        </section>
        }});

        let with_icons_no_label = with_raw_code!(with_icons_no_label { html! {
        <section>
            <MatTabBar>
                <MatTab icon="accessibility" indicator_icon="donut_large" is_fading_indicator=true />
                <MatTab icon="exit_to_app" indicator_icon="donut_large" is_fading_indicator=true />
                <MatTab icon="camera" indicator_icon="donut_large" is_fading_indicator=true />
            </MatTabBar>
        </section>
        }});

        html! {<main class="tab-component">
            <Codeblock title="Standard" code_and_html=standard max_width=100 />
            <Codeblock title="Fading indicator" code_and_html=fading_indicator max_width=100 />
            <Codeblock title="Min Width Tab" code_and_html=min_width_tab max_width=100 />
            <Codeblock title="Min Width Indicator" code_and_html=min_width_indicator max_width=100 />
            <Codeblock title="Min Width Tab - Min Width Indicator" code_and_html=min_width_tab_indicator max_width=100 />
            <Codeblock title="With Icons" code_and_html=with_icons max_width=100 />
            <Codeblock title="Only Icons" code_and_html=only_icons max_width=100 />
            <Codeblock title="Image Icons" code_and_html=image_icons max_width=100 />
            <Codeblock title="With Icons - Stacked" code_and_html=with_icons_stacked max_width=100 />
            <Codeblock title="With Icons - Stacked - Min Width Indicator" code_and_html=with_icons_stacked_min_width max_width=100 />
            <Codeblock title="Scrolling" code_and_html=scrolling max_width=100 />
            <Codeblock title="Scrolling - Width Icons - Stacked - Min Width Indicator" code_and_html=scrolling_with_icons_stacked_min_width max_width=100 />
            <Codeblock title="With Icons" code_and_html=with_icons_no_label max_width=100 />
        </main>}
    }
}
