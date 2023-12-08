use std::rc::Rc;
use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Message {
    pub data: String,
}

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
