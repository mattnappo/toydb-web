use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
    use reqwest::blocking::Client;
    use std::collections::HashMap;
    use std::error::Error;

    const API_ENDPOINT: &str = "http://localhost:3000/api";

    /// Execute a select query on the db
    pub fn select(req: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
        let resp = Client::new()
            .post(API_ENDPOINT)
            .json(req)
            .send()?
            .json::<HashMap<String, String>>()?;
        Ok(resp)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_SELECT1: &str = r#"{"jsonrpc":"2.0","id":"id","method":"select","params":{"db_name":"people", "table_name": "friends"}}"#;
        const TEST_SELECT2: &str = r#"{"jsonrpc":"2.0","id":"id","method":"select","params":{"db_name":"people", "table_name": "friends", "filter": {"Eq": [{ "Col": "Age"}, {"Val": {"I    nteger": 18}}]}}}"#;

        fn test_select() {
            let s = select(TEST_SELECT1).unwrap();
            println!("s1: {s:#?}");

            let s = select(TEST_SELECT2).unwrap();
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

struct TextState {
    text: String,
}

#[function_component(TextInput)]
fn text_input(props: &Props) -> Html {
    let state = use_state(|| TextState { text: "".into() });

    let onchange = {
        let state = state.clone();
        move |event: Event| {
            let target = event.target().unwrap();
            //println!("{:?}", target);
            let input = target.unchecked_into::<HtmlInputElement>();
            state.set(TextState {
                text: input.value(),
            });
        }
    };

    //let select = Callback::from(|_| resource::select(&state.text));

    html! {
        <div>
            <input type="text" name={props.name.clone()} onchange={onchange} />
            //<button onclick={select}>{"select"}</button>
            <p>{state.text.clone()}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
