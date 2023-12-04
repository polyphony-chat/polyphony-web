mod app;

use chorus::types::Snowflake;
use chorus::UrlBundle;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::app::Register;

fn main() {
    dioxus_web::launch(App);
}

/// Tuple of a [`UrlBundle`] and a [`Snowflake`], where the [`Snowflake`] is the ID of the User and
/// the [`UrlBundle`] are the URLs of the Instance.
pub type GlobalIdentifier = (UrlBundle, Snowflake);

#[derive(Routable, Clone)]
enum Route {
    #[route("/register")]
    Register {},
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
