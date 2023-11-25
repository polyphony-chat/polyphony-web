use crate::message::{self, Message};
use crate::screen::{self, Screen};
use crate::GlobalIdentifier;

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use chorus::instance::{ChorusUser, Instance};
use chorus::types::Guild;
use chorus::UrlBundle;
use yew::{Component, Context, Html};

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
    pub screens: HashSet<Screen>,
    pub screen: Screen,
}

impl Component for Client {
    type Message = Message;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        println!("Hello, world!");
        eprintln!("aaa");
        match &self.screen {
            Screen::Login(login) => login.view(self.data.clone(), ctx),
            Screen::Dashboard(dashboard) => dashboard.view(),
            Screen::Crash(crash) => crash.view(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Login(message) => {
                let Screen::Login(_) = &mut self.screen else {
                    return false;
                };
                message::LoginMessage::update(self, message)
            }
            Message::Dashboard(_) => todo!(),
            Message::Crash(_) => todo!(),
        }
    }
}
