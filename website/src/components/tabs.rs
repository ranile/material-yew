use yew::prelude::*;
use yew_material_components::{MatTab, MatTabBar};

pub struct Tabs {
    link: ComponentLink<Self>,
}

impl Component for Tabs {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false    }

    fn change(&mut self, _props: Self::Properties) -> bool { false }

    fn view(&self) -> Html {
        html! {<main class="tab-component">
            <MatTabBar>
                <MatTab label="one" />
                <MatTab label="two" />
                <MatTab label="three" />
            </MatTabBar>

            <section>
                <h2>{"Fading indicator"}</h2>
                <MatTabBar>
                    <MatTab is_fading_indicator=true label="one" />
                    <MatTab is_fading_indicator=true label="two" />
                    <MatTab is_fading_indicator=true label="three" />
                </MatTabBar>
            </section>

            <section>
                <h2>{"Min Width Tab"}</h2>
                <MatTabBar>
                    <MatTab min_width=true label="one" />
                    <MatTab min_width=true label="two" />
                    <MatTab min_width=true label="three" />
                </MatTabBar>
            </section>

            <section>
                <h2>{"Min Width Indicator"}</h2>
                <MatTabBar>
                    <MatTab is_min_width_indicator=true label="one" />
                    <MatTab is_min_width_indicator=true label="two" />
                    <MatTab is_min_width_indicator=true label="three" />
                </MatTabBar>
            </section>

            <section>
                <h2>{"Min Width Tab - Min Width Indicator"}</h2>
                <MatTabBar>
                    <MatTab is_min_width_indicator=true min_width=true label="one" />
                    <MatTab is_min_width_indicator=true min_width=true label="two" />
                    <MatTab is_min_width_indicator=true min_width=true label="three" />
                </MatTabBar>
            </section>

            <section>
                <h2>{"With Icons"}</h2>
                <MatTabBar>
                    <MatTab icon="accessibility" label="one" />
                    <MatTab icon="exit_to_app" label="two" />
                    <MatTab icon="camera" label="three" />
                </MatTabBar>
            </section>

            <section>
                <h2>{"Only Icons"}</h2>
                <MatTabBar>
                    <MatTab icon="accessibility" />
                    <MatTab icon="exit_to_app" />
                    <MatTab icon="camera" />
                </MatTabBar>
            </section>

            <section>
                <h2>{"Image Icons"}</h2>
                <MatTabBar>
                    <MatTab>
                        <svg slot="icon" width="10px" height="10px"><circle r="5px" cx="5px" cy="5px" color="red"></circle></svg>
                    </MatTab>
                    <MatTab>
                        <svg slot="icon" width="10px" height="10px"><rect width="10px" height="10px" color="green"></rect></svg>
                    </MatTab>
                    <MatTab>
                        <svg slot="icon" width="10px" height="10px"><rect width="10px" height="10px" color="green"></rect></svg>
                    </MatTab>
                </MatTabBar>
            </section>

            <section>
                <h2>{"With Icons - Stacked"}</h2>
                <MatTabBar>
                    <MatTab icon="accessibility" label="one" stacked=true />
                    <MatTab icon="exit_to_app" label="two" stacked=true />
                    <MatTab icon="camera" label="three" stacked=true />
                </MatTabBar>
            </section>

            <section>
                <h2>{"With Icons - Stacked - Min Width Indicator"}</h2>
                <MatTabBar>
                    <MatTab icon="accessibility" label="one" stacked=true is_min_width_indicator=true />
                    <MatTab icon="exit_to_app" label="two" stacked=true is_min_width_indicator=true />
                    <MatTab icon="camera" label="three" stacked=true is_min_width_indicator=true />
                </MatTabBar>
            </section>

            <section>
                <h2>{"Scrolling"}</h2>
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

            <section>
                <h2>{"Scrolling - Width Icons - Stacked - Min Width Indicator"}</h2>
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

            <section>
                <h2>{"With Icons"}</h2>
                <MatTabBar>
                    <MatTab icon="accessibility" indicator_icon="donut_large" is_fading_indicator=true />
                    <MatTab icon="exit_to_app" indicator_icon="donut_large" is_fading_indicator=true />
                    <MatTab icon="camera" indicator_icon="donut_large" is_fading_indicator=true />
                </MatTabBar>
            </section>
        </main>}
    }
}
