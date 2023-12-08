use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const TEST_SELECT1: &str = r#"{"jsonrpc":"2.0","id":"id","method":"select","params":{"db_name":"people", "table_name": "friends"}}"#;

#[derive(Debug, Default)]
struct Model {
    x: i64,
    y: i64,
}

impl Model {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

mod protocol {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Serialize, Deserialize)]
    pub struct InsertReq {
        db_name: String,
        table_name: String,
        values: Vec<Value>,
    }
    #[derive(Serialize, Deserialize)]
    pub struct SelectReq {
        db_name: String,
        table_name: String,
        //filter: Option<Cmp>,
    }
}

mod resource {
    use super::protocol;
    use reqwest::Client;
    use std::collections::HashMap;
    use std::error::Error;

    const API_ENDPOINT: &str = "http://localhost:3000/api";

    /// Execute a select query on the db
    pub async fn select(req: &str) -> Result<String, Box<dyn Error>> {
        let resp = Client::new()
            .post(API_ENDPOINT)
            .body(req.to_string())
            .send()
            .await?
            .text()
            .await?;
        //.json::<HashMap<String, String>>()
        //.await?;
        println!("res: {resp:?}");
        Ok(resp)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_SELECT1: &str = r#"{"jsonrpc":"2.0","id":"id","method":"select","params":{"db_name":"people", "table_name": "friends"}}"#;
        const TEST_SELECT2: &str = r#"{"jsonrpc":"2.0","id":"id","method":"select","params":{"db_name":"people", "table_name": "friends", "filter": {"Eq": [{ "Col": "Age"}, {"Val": {"I    nteger": 18}}]}}}"#;

        #[tokio::test]
        async fn test_select() {
            let s = select(TEST_SELECT1).await.unwrap();
            println!("s1: {s:#?}");

            let s = select(TEST_SELECT2).await.unwrap();
            println!("s2: {s:#?}");
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model::default());

    let onclick_x = {
        let state = state.clone();
        move |_| state.set(Model::new(state.x + 1, state.y))
    };
    let onclick_y = {
        let state = state.clone();
        move |_| state.set(Model::new(state.x, state.y + 1))
    };

    //let select = |_| resource::select();

    html!(
        <div>
            <button onclick={onclick_x}>{"add one to x"}</button>
            <button onclick={onclick_y}>{"add one to y"}</button>
            //<button onclick={select}>{"select"}</button>
            <p>{format!("state: {:?}", state)}</p>
            <TextInput name="query" />
        </div>
    )
}

#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
}

#[derive(Default)]
struct State {
    query: String,
    results: String,
}

#[function_component(TextInput)]
fn text_input(props: &Props) -> Html {
    let state = use_state(State::default);

    let onchange = {
        let state = state.clone();
        move |event: Event| {
            let target = event.target().unwrap();
            //println!("{:?}", target);
            let input = target.unchecked_into::<HtmlInputElement>();
            state.set(State {
                query: input.value(),
                results: state.results.clone(),
            });
        }
    };

    let select = {
        let state = state.clone();
        // Make request
        wasm_bindgen_futures::spawn_local(async move {
            let res = resource::select(&state.query).await.unwrap();
            state.set(State {
                query: state.query.clone(),
                results: res,
            })
        })

        // Update state
    };

    /*
    html! {
        <div>
            <input type="text" name={props.name.clone()} onchange={onchange} />
            <button onclick={select}>{"select"}</button>
            <p>{state.results.clone()}</p>
        </div>
    }
    */

    html!()
}

enum ComponentLoadingStage {
    Loading,
    Success,
    Error,
}

impl Default for ComponentLoadingStage {
    fn default() -> Self {
        Self::Loading
    }
}

#[derive(Default)]
struct Query {
    results: String,
    loading_stage: ComponentLoadingStage,
}

impl Component for Query {
    type Message = Result<String, Box<dyn std::error::Error>>;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Query::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.loading_stage {
            ComponentLoadingStage::Loading => {
                html! { <h1>{"querying..."}</h1>}
            }
            /*
            ComponentLoadingStage::Success => self
                .lesson_plans
                .iter()
                .map(|lp| {
                    html! { <ViewLessonPlan lesson_plan={lp.clone()} /> }
                })
                .collect::<Html>(),
            */
            ComponentLoadingStage::Success => html! { <p>{self.results.clone()}</p> },
            ComponentLoadingStage::Error => {
                html! { <h1>{ "error" }</h1>}
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = resource::select(TEST_SELECT1).await;
                link.send_message(result);
            });
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(results) => {
                self.results = results;
                self.loading_stage = ComponentLoadingStage::Success;
            }
            Err(_) => {
                self.loading_stage = ComponentLoadingStage::Error;
            }
        }
        true
    }
}

fn main() {
    //yew::Renderer::<App>::new().render();
    yew::Renderer::<Query>::new().render();
}
