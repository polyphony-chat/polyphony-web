mod app;
mod stores;

use chorus::types::Snowflake;
use chorus::UrlBundle;
use leptos::*;
use log::*;

use crate::app::Register;
use crate::stores::{InstanceStore, UserStore};

#[component]
fn App() -> impl IntoView {
    let instance_store = InstanceStore::default();
    provide_context(instance_store.instances);
    provide_context(create_signal(UserStore::default()));
    debug!("Rendering the App view");
    view! {
        <Register/>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    debug!("wasm_logger initialized!");
    debug!("Starting App...");
    leptos::mount_to_body(|| view! { <App/> })
}

/// Tuple of a [`UrlBundle`] and a [`Snowflake`], where the [`Snowflake`] is the ID of the User and
/// the [`UrlBundle`] are the URLs of the Instance.
pub type GlobalIdentifier = (UrlBundle, Snowflake);
