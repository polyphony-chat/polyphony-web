mod client;
mod infobar;
mod message;
mod screen;

use chorus::types::Snowflake;
use chorus::UrlBundle;

use yew::prelude::*;

use crate::client::Client;
use crate::infobar::Infobar;

/// <-- Yew boilerplate -->

fn main() {
    yew::Renderer::<App>::new().render();
}

/// <-- Data Structures -->

#[function_component]
fn App() -> Html {
    env_logger::init();
    html! {
        <>
            <Client />
            <Infobar />
        </>
    }
}

/// Tuple of a [`UrlBundle`] and a [`Snowflake`], where the [`Snowflake`] is the ID of the User and
/// the [`UrlBundle`] are the URLs of the Instance.
pub type GlobalIdentifier = (UrlBundle, Snowflake);
