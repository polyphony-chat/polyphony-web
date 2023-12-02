use chorus::instance::Instance;
use chorus::UrlBundle;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub(crate) struct AuthenticationStore {
    instances: Vec<(UrlBundle, Instance)>,
}
