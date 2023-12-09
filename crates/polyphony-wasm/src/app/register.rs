use leptos::*;

#[component]
pub fn Register() -> impl IntoView {
    view! {
        <form>
            <input type="email" id="mail" name="email"/><br/>
            <input type="password" id="pass" name="pass"/><br/>
            <input type="text" id="iurl" name="iurl"/><br/>
            <button type="submit" id="submitbutton"/>
        </form>
    }
}
