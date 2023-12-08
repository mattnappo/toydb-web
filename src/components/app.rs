use super::dataview::Dataview;
use crate::context::*;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <MessageProvider>
            <div>
                <h1>{"ToyDB Web Client"}</h1>
                <Dataview />
            </div>
        </MessageProvider>
    }
}
