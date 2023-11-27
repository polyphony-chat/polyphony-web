use yew::prelude::*;

struct RegisterProps {}

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <form class="register">
            <h1>{"Register"}</h1>
            <label for ="username">{"Username"}</label>
            <input type="text" id="username" required=true/>
            <label for ="password">{"Password"}</label>
            <input type="password" id="password" required=true/>
            <label for ="email">{"Email"}</label>
            <input type="email" id="email"/>
            <label for="consent">{"I agree to the terms of service."}</label>
            <input type="checkbox" id="consent" required=true/>
            <label for="age">{"I am at least 16 years old."}</label>
            <input type="checkbox" id="age" required=true/>
            <input type="submit" value="Sign Up" required=true/>
        </form>
    }
}
