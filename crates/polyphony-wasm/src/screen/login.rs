use yew::{html, Html};

#[derive(Debug, Default)]
pub struct LoginScreen;

impl LoginScreen {
    pub fn view(&self) -> Html {
        html! {
            <div>
                <h2 class={"text-3xl font-bold underline"}>{"Login Page"}</h2>
            </div>
        }
    }
}
