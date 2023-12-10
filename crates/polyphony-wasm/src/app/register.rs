use chorus::errors::ChorusResult;
use chorus::instance::Instance;
use chorus::types::RegisterSchema;
use leptos::*;
use log::*;

#[component]
pub fn Register() -> impl IntoView {
    let (mail, set_mail) = create_signal(String::new());
    let (pass, set_pass) = create_signal(String::new());
    let (url, set_url) = create_signal(String::new());

    let submit = create_action(|input: &(String, String, String)| {
        let input = input.to_owned();
        async move { send_register(&input).await }
    });
    debug!("Rendering Register component");
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = (url.get().to_string(), pass.get().to_string(), mail.get().to_string());
            submit.dispatch(input);
        }>
            <label for="mail">Email:</label>
            <input class="border-2 border-black text-black" type="email" id="mail" name="email" on:input=move |ev| {
                set_mail.set(event_target_value(&ev));
            } prop:value=mail/><br/>
            <label for="pass">Password:</label>
            <input class="border-2 border-black text-black" type="password" id="pass" name="pass" on:input=move |ev| {
                set_pass.set(event_target_value(&ev));
            } prop:value=pass/><br/>
            <label for="iurl">Instance URL:</label>
            <input class="border-2 border-black text-black" type="text" id="iurl" name="iurl" on:input=move |ev| {
                set_url.set(event_target_value(&ev));
            } prop:value=url/><br/>
            <button type="submit" id="submitbutton">Submit</button>
        </form>
    }
}

async fn send_register(input: &(String, String, String)) -> ChorusResult<Instance> {
    let reg = RegisterSchema {
        username: input.2.clone(),
        password: Some(input.1.clone()),
        consent: true,
        email: Some(input.2.clone()),
        ..Default::default()
    };
    let instance = Instance::from_root_url(&input.0).await;
    debug!("Got instance: {:?}", instance.clone().unwrap());
    instance
}
