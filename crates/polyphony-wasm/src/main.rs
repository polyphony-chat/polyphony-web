mod app;

use chorus::types::Snowflake;
use chorus::UrlBundle;
use leptos::*;
use log::*;

use crate::app::Register;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <Register/>
        <button on:click=move |_| {
            set_count.set(count.get() + 1);
        }>
        "Click me counter: "
        {move || count.get()}
        </button>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    debug!("wasm_logger initialized!");
    debug!("Starting App...");
    leptos::mount_to_body(|| view! { <App/>})
}

/// Tuple of a [`UrlBundle`] and a [`Snowflake`], where the [`Snowflake`] is the ID of the User and
/// the [`UrlBundle`] are the URLs of the Instance.
pub type GlobalIdentifier = (UrlBundle, Snowflake);
