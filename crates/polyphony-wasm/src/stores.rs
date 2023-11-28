use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
pub(crate) struct AuthenticationStore {}
