use crate::context::MessageContext;
use crate::resource;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default)]
struct State {
    query: String,
}

#[derive(Properties, PartialEq)]
pub struct Props;

#[function_component(Dataview)]
pub fn data_view(_props: &Props) -> Html {
    let state = use_state(State::default);
    let ctx = use_context::<MessageContext>().unwrap();

    let textbox_onchange = {
        let state = state.clone();
        move |event: Event| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            state.set(State {
                query: input.value(),
            });
        }
    };

    let ctxc = ctx.clone();
    let onclick = {
        let state = state.clone();
        move |_| {
            // Fetch the data
            let query = state.query.clone();

            let ctxc = ctxc.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = resource::select(&query).await;
                ctxc.dispatch(result.unwrap());
            });

            // Update the state
            state.set(State {
                query: state.query.clone(),
            });
        }
    };

    html! {
        <div style="width: 800px;">
            <textarea rows=20 cols=80 type="text" name="query" onchange={textbox_onchange} />
            <br />
            <button {onclick}>{"execute query"}</button>
            <hr />
            <code>
                {ctx.data.to_owned().clone()}
            </code>
        </div>
    }
}
