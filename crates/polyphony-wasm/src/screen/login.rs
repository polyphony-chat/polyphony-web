use std::sync::{Arc, RwLock};

use yew::{html, Html};

use crate::client::Data;

#[derive(Debug, Default)]
pub struct LoginScreen;

impl LoginScreen {
    pub fn view(&self, data: Arc<RwLock<Data>>) -> Html {
        html! {
            <div>
                <h2 class={"text-3xl font-bold underline"}>{"Login Page"}</h2>
            </div>
        }
    }
}
