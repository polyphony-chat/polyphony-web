use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Store, PartialEq, Clone, Default)]
struct RegisterProps {
    username: String,
    password: String,
    email: String,
    consent: bool,
    age: bool,
}

#[function_component(Register)]
pub fn register() -> Html {
    let (props, dispatch) = use_store::<RegisterProps>();
    let onchange_username = {
        let dispatch_clone = dispatch.clone();
        move |value: Event| {
            dispatch_clone.reduce_mut(|state| {
                state.username = value.target_unchecked_into::<HtmlInputElement>().value()
            })
        }
    };
    let onchange_password = {
        let dispatch_clone = dispatch.clone();
        move |value: Event| {
            dispatch_clone.reduce_mut(|state| {
                state.password = value.target_unchecked_into::<HtmlInputElement>().value()
            })
        }
    };
    let onchange_email = {
        let dispatch_clone = dispatch.clone();
        move |value: Event| {
            dispatch_clone.reduce_mut(|state| {
                state.email = value.target_unchecked_into::<HtmlInputElement>().value()
            })
        }
    };
    html! {
        <form class="register">
            <h1>{"Register"}</h1>
            <label for ="username">{"Username"}</label>
            <input type="text" id="username" required=true onchange={onchange_username} />
            <label for ="password">{"Password"}</label>
            <input type="password" id="password" required=true onchange={onchange_password}/>
            <label for ="email">{"Email"}</label>
            <input type="email" id="email" onchange={onchange_email}/>
            <label for="consent">{"I agree to the terms of service."}</label>
            <input type="checkbox" id="consent" required=true/>
            <label for="age">{"I am at least 16 years old."} </label>
            <input type="checkbox" id="age" required=true/>
            <input type="submit" value="Sign Up" required=true/>
            <p>{props.age}</p>
        </form>
    }
}
