use serde_json::json;
use std::error::Error;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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

/* +------------------------------------+ */
/* |             MAIN CODE              | */
/* +------------------------------------+ */

/*
#[derive(Properties, PartialEq)]
pub struct Props {
    name: String,
}

#[derive(Default)]
struct QueryState {
    query: String,
}

/// A textbox and a button
#[function_component(Query)]
fn query(props: &Props) -> Html {
    let state = use_state(QueryState::default);

    let onchange = {
        let state = state.clone();
        move |event: Event| {
            let target = event.target().unwrap();
            //println!("{:?}", target);
            let input = target.unchecked_into::<HtmlInputElement>();
            state.set(QueryState {
                query: input.value(),
            });
        }
    };

    html! {
        <div>
            <input type="text" name={props.name.clone()} onchange={onchange} />
            // <button onclick={select}>{"select"}</button>
        </div>
    }
}
*/

#[derive(Clone)]
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

//#[derive(Default)]
//struct Dataview {
//    data: String,
//    loading_stage: ComponentLoadingStage,
//}

#[derive(Default)]
struct State {
    query: String,
    loading_stage: ComponentLoadingStage,
}

#[derive(Properties, PartialEq)]
struct Props;

#[derive(PartialEq, Clone)]
struct Message {
    data: String,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SelectRes {
    jsonrpc: String,
    id: String,
    method: String,
    params: Vec<serde_json::Value>,
}

fn pretty(s: &str) -> String {
    let data = serde_json::from_str::<SelectRes>(s).unwrap();
    let json = json!( { "response": data } );
    format!("{json}")
}

use std::rc::Rc;

impl Reducible for Message {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Message { data: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<Message>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn MessageProvider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(|| Message {
        data: "No data yet.".to_string(),
    });

    html! {
        <ContextProvider<MessageContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}

#[function_component]
fn Dataview(_props: &Props) -> Html {
    let state = use_state(State::default);
    let ctx = use_context::<MessageContext>().unwrap();

    let textbox_onchange = {
        let state = state.clone();
        move |event: Event| {
            let target = event.target().unwrap();
            let input = target.unchecked_into::<HtmlInputElement>();
            state.set(State {
                query: input.value(),
                loading_stage: state.loading_stage.clone(),
            });
        }
    };

    let ctx2 = ctx.clone();
    let onclick = {
        let state = state.clone();
        move |_| {
            // Fetch the data
            let q = state.query.clone();

            let msg_ctx1 = ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = resource::select(&q.to_string()).await;
                msg_ctx1.dispatch(result.unwrap());
            });

            //let message = msg_ctx.data.to_owned();

            // Update the state
            state.set(State {
                query: state.query.clone(),
                loading_stage: state.loading_stage.clone(),
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
                {ctx2.data.to_owned().clone()}
            </code>
        </div>
    }
}

/*
struct OldDataview;
impl Component for OldDataview {
    type Message = Result<String, Box<dyn std::error::Error>>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        OldDataview
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onchange = {
            let
            move |event: Event| {
                let target = event.target().unwrap();
                //println!("{:?}", target);
                let input = target.unchecked_into::<HtmlInputElement>();
                state.set(QueryState {
                    query: input.value(),
                });
            }
        };

        let data = match self.loading_stage {
            ComponentLoadingStage::Loading => {
                html! { <h1>{"querying..."}</h1>}
            }
            ComponentLoadingStage::Success => html! { <p>{self.data.clone()}</p> },
            ComponentLoadingStage::Error => {
                html! { <h1>{ "error" }</h1>}
            }
        };

        html! {
            <div>
                <input type="text" name="query" onchange={onchange} />
                //<button onclick={select}>{"select"}</button>
                {data}
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = resource::select(&self.query).await;
                link.send_message(result);
            });
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Ok(data) => {
                self.data = data;
                self.loading_stage = ComponentLoadingStage::Success;
            }
            Err(_) => {
                self.loading_stage = ComponentLoadingStage::Error;
            }
        }
        true
    }
}

*/

#[function_component(App)]
fn app() -> Html {
    html! {
        <MessageProvider>
            <div>
                <h1>{"ToyDB Web Client"}</h1>
                <Dataview />
            </div>
        </MessageProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
