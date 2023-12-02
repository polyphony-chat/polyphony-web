use std::borrow::BorrowMut;
use std::rc::Rc;

use crate::stores::AuthenticationStore;
use chorus::instance::Instance;
use chorus::UrlBundle;
use yew::prelude::*;
use yew_router::scope_ext::RouterScopeExt;
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

impl RegisterPage {
    fn individual_urls_set(&self) -> bool {
        self.url_api.is_some() && self.url_wss.is_some() && self.url_cdn.is_some()
    }
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            RegisterPageMsg::AttemptRegister => {
                // TODO: Check consent and of_age before proceeding with registration.

                let dispatch = self.dispatch.clone();
                let set_error_callback = ctx.link().callback(RegisterPageMsg::SetError);
                let navigator = ctx.link().navigator().unwrap();
                let url_bundle = match self.individual_urls_set() {
                    true => UrlBundle::new(
                        self.url_api.as_ref().unwrap().to_string(),
                        self.url_wss.as_ref().unwrap().to_string(),
                        self.url_wss.as_ref().unwrap().to_string(),
                    ),
                    false => todo!(), // TODO: Get URLs from .well-known,
                };

                wasm_bindgen_futures::spawn_local(async move {});
                true
            }
            RegisterPageMsg::SetError(_) => todo!(),
            RegisterPageMsg::ToggleConsent => {
                self.consent = !self.consent;
                false
            }
            RegisterPageMsg::ToggleAgeConfirm => {
                self.of_age = !self.of_age;
                false
            }
            RegisterPageMsg::UpdateAuth(data) => {
                self.state = data;
                false
            }
            RegisterPageMsg::UpdateUrlApi(data) => {
                self.url_api = Some(data);
                false
            }
            RegisterPageMsg::UpdateUrlCdn(data) => {
                self.url_cdn = Some(data);
                false
            }
            RegisterPageMsg::UpdateUrlWss(data) => {
                self.url_wss = Some(data);
                false
            }
            RegisterPageMsg::UpdateUrlBase(data) => {
                self.url_base = Some(data);
                false
            }
            RegisterPageMsg::UpdatePassword(data) => {
                self.password = data;
                false
            }
            RegisterPageMsg::UpdateEmail(data) => {
                self.email = data;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}
