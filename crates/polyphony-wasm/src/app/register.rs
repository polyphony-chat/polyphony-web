use std::rc::Rc;

use crate::stores::AuthenticationStore;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Default)]
pub(crate) struct RegisterPage {
    state: Rc<AuthenticationStore>,
    dispatch: Dispatch<AuthenticationStore>,

    url_api: Option<AttrValue>,
    url_wss: Option<AttrValue>,
    url_cdn: Option<AttrValue>,
    url_base: Option<AttrValue>,

    email: AttrValue,
    password: AttrValue,
    consent: bool,
    of_age: bool,
}

pub(crate) enum RegisterPageMsg {
    AttemptRegister,
    SetError(String),
    ToggleConsent,
    ToggleAgeConfirm,
    UpdateAuth(Rc<AuthenticationStore>),
    UpdateUrlApi(AttrValue),
    UpdateUrlCdn(AttrValue),
    UpdateUrlWss(AttrValue),
    UpdateUrlBase(AttrValue),
    UpdatePassword(AttrValue),
    UpdateEmail(AttrValue),
}

impl Component for RegisterPage {
    type Message = RegisterPageMsg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(RegisterPageMsg::UpdateAuth);
        let dispatch = Dispatch::<AuthenticationStore>::subscribe(callback);

        Self {
            state: dispatch.get(),
            dispatch,
            ..Default::default()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}
