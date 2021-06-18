use yew::prelude::*;

use crate::components::todo::Todo;
use yew_router::{components::RouterAnchor, router::Router, Switch};

pub type Anchor = RouterAnchor<AppRoute>;

pub struct App {}
pub enum Msg {}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/todo"]
    Home,
    #[to = "/"]
    Todo,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                </div>
                <div>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::Todo => {
                                    html! {
                                        <div>
                                            <Todo />
                                        </div>}
                                }
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <div class="banner">
                                                {"you can use all functions if you sign-up"}
                                            </div>
                                            <Todo />
                                        </div>}
                                }
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}
