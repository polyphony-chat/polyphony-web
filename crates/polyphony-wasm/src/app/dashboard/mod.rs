use yew::prelude::*;

#[derive(Clone, Default, PartialEq, Properties)]
pub struct Dashboard;

#[derive(Properties, PartialEq)]
pub struct DashboardProps;

pub enum DashboardMsg {}

impl Component for Dashboard {
    type Message = DashboardMsg;

    type Properties = DashboardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="dash">
                <h1>{"Dashboard"}</h1>
                <p>{":)"}</p>
            </div>
        }
    }
}
