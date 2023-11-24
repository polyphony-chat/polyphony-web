use std::sync::{Arc, RwLock};

use chorus::instance::ChorusUser;
use chorus::types::Guild;
use yew::Html;

use crate::client::Data;
use crate::GlobalIdentifier;

#[derive(Debug, Default, Clone)]
pub struct DashboardScreen {
    pub current_user: Option<ChorusUser>,
    pub data: Arc<RwLock<Data>>,
    pub guilds: Vec<(GlobalIdentifier, Guild)>,
}

impl DashboardScreen {
    pub fn view(&self) -> Html {
        todo!()
    }
}
