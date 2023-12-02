mod dashboard;
mod info;
mod login;
mod register;

use chorus::instance::ChorusUser;
use hashbrown::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::GlobalIdentifier;

use self::dashboard::Dashboard;
use self::info::Info;
use self::login::Login;
use self::register::RegisterPage;

#[function_component]
pub fn App() -> Html {
    env_logger::init();
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// Client-relevant data about the current user.
#[derive(Clone, Default)]
pub struct AccountWrapper {
    /// The current user.
    pub user: Option<ChorusUser>,
    /// Whether the user is logged in, i.e. has a valid session.
    pub active: bool,
    /// The user's global identifier.
    pub global_identifier: GlobalIdentifier,
}

impl PartialEq for AccountWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.active == other.active && self.global_identifier == other.global_identifier
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct ClientSettings {
    language: String,
}

#[derive(PartialEq, Store, Default)]
pub struct State {
    dashboard: Dashboard,
}

#[derive(PartialEq, Store, Default)]
pub struct TabSyncedState {
    client_settings: ClientSettings,
    users: HashMap<GlobalIdentifier, AccountWrapper>,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/dashboard")]
    Dashboard,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Info /> },
        Route::Login => html! { <Login /> },
        Route::Register => html! { <RegisterPage /> },
        Route::Dashboard => html! { <Dashboard /> },
    }
}
