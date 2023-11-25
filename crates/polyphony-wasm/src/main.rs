mod app;

use chorus::types::Snowflake;
use chorus::UrlBundle;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

/// Tuple of a [`UrlBundle`] and a [`Snowflake`], where the [`Snowflake`] is the ID of the User and
/// the [`UrlBundle`] are the URLs of the Instance.
pub type GlobalIdentifier = (UrlBundle, Snowflake);
