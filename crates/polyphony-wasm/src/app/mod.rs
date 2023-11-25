mod dashboard;
mod info;
mod login;
mod register;

use yew::prelude::*;
use yew_router::prelude::*;

use self::dashboard::Dashboard;
use self::info::Info;
use self::login::Login;
use self::register::Register;

#[function_component]
pub fn App() -> Html {
    env_logger::init();
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
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
        Route::Register => html! { <Register /> },
        Route::Dashboard => html! { <Dashboard /> },
    }
}
