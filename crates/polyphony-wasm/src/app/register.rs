use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct RegisterProps<'a> {
    url_api: &'a str,
}

#[allow(non_snake_case)]
pub fn Register<'a>(cx: Scope<'a, RegisterProps<'a>>) -> Element {
    cx.render(rsx!(
        p {
            "register {cx.props.url_api}"
        }
    ))
}
