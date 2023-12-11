use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use hashbrown::HashMap;
use leptos::RwSignal;

use crate::GlobalIdentifier;

#[derive(Clone, Debug, Default)]
pub(crate) struct InstanceStore {
    pub(crate) instances: RwSignal<HashMap<UrlBundle, Instance>>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct UserStore {
    pub(crate) users: HashMap<GlobalIdentifier, ChorusUser>,
}
