use yew::prelude::*;

use crate::components::todo::Todo;
use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew_router::{components::RouterAnchor, router::Router, Switch};

pub type Anchor = RouterAnchor<AppRoute>;

pub struct App {
    link: ComponentLink<Self>,
    tadas: Option<Vec<Tada>>,
    fetch_task: Option<FetchTask>,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Tada>, anyhow::Error>),
}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/todo"]
    Home,
    #[to = "/"]
    Todo,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tada {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            link,
            tadas: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.tadas = None;
                let req = Request::get("https://jsonplaceholder.typicode.com/todos")
                    .body(Nothing)
                    .expect("can make req to jsonplaceholder");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Tada>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.tadas = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
  Z

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
