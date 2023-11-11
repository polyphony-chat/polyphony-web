use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            <h2 class={"text-3xl font-bold underline"}>{"Hello, World!"}</h2>
        </div>
    }
}
