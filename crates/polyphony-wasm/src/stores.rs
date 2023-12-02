use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

use crate::GlobalIdentifier;

#[derive(Store, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub(crate) struct AuthenticationStore {
    pub(crate) instances: Vec<(UrlBundle, Instance)>,
    /// Tuple (GlobalIdentifier, Token)
    pub(crate) identities: Vec<(GlobalIdentifier, String)>,
}

#[derive(Store, Default, PartialEq)]
pub(crate) struct UserStore {
    users: Vec<(GlobalIdentifier, ChorusUser)>,
}
