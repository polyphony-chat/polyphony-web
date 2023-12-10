use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use hashbrown::HashMap;

use crate::GlobalIdentifier;

#[derive(Clone, Debug, Default)]
pub(crate) struct AuthenticationStore {
    pub(crate) instances: HashMap<UrlBundle, Instance>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct UserStore {
    pub(crate) users: HashMap<GlobalIdentifier, ChorusUser>,
}
