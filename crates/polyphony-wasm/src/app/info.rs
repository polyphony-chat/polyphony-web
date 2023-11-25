use yew::prelude::*;

#[function_component(Info)]
pub fn info() -> Html {
    html! {
        <div class="info">
            <h1>{"Polyphony"}</h1>
            <p>{"Polyphony is cool"}</p>
        </div>
    }
}
