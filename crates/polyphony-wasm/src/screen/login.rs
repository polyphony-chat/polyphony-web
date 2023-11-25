use std::sync::{Arc, RwLock};

use yew::{html, Context, Html};

use crate::client::{Client, Data};
use crate::message::{LoginMessage, Message};

#[derive(Debug, Default)]
pub struct LoginScreen {
    pub text: String,
}

impl LoginScreen {
    pub fn view(&self, data: Arc<RwLock<Data>>, ctx: &Context<Client>) -> Html {
        let onclick = ctx
            .link()
            .callback(|_| Message::Login(LoginMessage::Test("hi".to_string())));
        html! {
            <div>
                <button {onclick}>{"Click me!"}</button>
                <p>{&self.text}</p>
            </div>
        }
    }
}
