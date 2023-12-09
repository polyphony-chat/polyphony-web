use chorus::instance::Instance;
use chorus::types::RegisterSchema;
use leptos::*;
use log::*;

#[component]
pub fn Register() -> impl IntoView {
    let (mail, set_mail) = create_signal(String::new());
    let (pass, set_pass) = create_signal(String::new());
    let (url, set_url) = create_signal(String::new());
    let once =
        create_resource(
            || (),
            async move |_| {
                send_register(
                    url.get().to_string(),
                    pass.get().to_string(),
                    mail.get().to_string(),
                )
                .await
            },
        );
    view! {
        <form>
            <input class="border-2 border-black text-black" type="email" id="mail" name="email" on:input=move |ev| {
                set_mail.set(event_target_value(&ev));
            } prop:value=mail/><br/>
            <input class="border-2 border-black text-black" type="password" id="pass" name="pass" on:input=move |ev| {
                set_pass.set(event_target_value(&ev));
            } prop:value=pass/><br/>
            <input class="border-2 border-black text-black" type="text" id="iurl" name="iurl" on:input=move |ev| {
                set_url.set(event_target_value(&ev));
            } prop:value=url/><br/>
            <button type="submit" id="submitbutton" on:click=move |_| {}>Submit</button>
        </form>
    }
}

async fn send_register(url: String, pass: String, mail: String) {
    let reg = RegisterSchema {
        username: mail.clone(),
        password: Some(pass),
        consent: true,
        email: Some(mail),
        ..Default::default()
    };
    let instance = Instance::from_root_url(&url).await.unwrap();
    debug!("{:?}", instance);
}
