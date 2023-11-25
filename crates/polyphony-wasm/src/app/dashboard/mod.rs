use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="dash">
            <h1>{"Dashboard"}</h1>
            <p>{":)"}</p>
        </div>
    }
}
