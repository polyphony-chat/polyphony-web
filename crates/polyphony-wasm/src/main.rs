mod message;
mod screen;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use chorus::instance::{ChorusUser, Instance};
use chorus::types::{Guild, Snowflake};
use chorus::UrlBundle;
use message::Message;
use screen::Screen;
use yew::prelude::*;

/// <-- Yew boilerplate -->

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <h2 class={"text-3xl font-bold underline"}>{"Hello, World!"}</h2>
        </div>
    }
}

/// <-- Data Structures -->

/// Tuple of a [`UrlBundle`] and a [`Snowflake`], where the [`Snowflake`] is the ID of the User and
/// the [`UrlBundle`] are the URLs of the Instance.
pub type GlobalIdentifier = (UrlBundle, Snowflake);

#[derive(Debug, Default, Clone)]
pub struct Data {
    pub instances: HashMap<UrlBundle, Instance>,
    pub url_to_bundle: HashMap<String, UrlBundle>,
    pub users: HashMap<GlobalIdentifier, ChorusUser>,
    pub dashboard: Option<screen::DashboardScreen>,
    pub guilds: HashMap<GlobalIdentifier, Guild>,
}

impl Data {
    pub fn url_bundle_to_urls(&mut self, bundle: &UrlBundle) {
        self.url_to_bundle
            .insert(bundle.api.clone(), bundle.clone());
        self.url_to_bundle
            .insert(bundle.wss.clone(), bundle.clone());
        self.url_to_bundle
            .insert(bundle.cdn.clone(), bundle.clone());
    }
}

#[derive(Debug, Default)]
pub struct Client {
    pub data: Arc<RwLock<Data>>,
    pub screen: Screen,
}

impl Component for Client {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }
}
