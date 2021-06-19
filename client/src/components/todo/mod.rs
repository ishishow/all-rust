use crate::components::todo_item::TodoItem;
use crate::model;
use serde_derive::Deserialize;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};

pub struct Todo {
    link: ComponentLink<Self>,
    list: Vec<model::TodoItem>,
    input: String,
    show_error: bool,
    user_name: String,
}

pub enum Msg {
    UpdateInput(String),
    AddTodoItem,
    DeleteItem(i32),
    SetUserName(String),
    MakeReq,
    Resp(Result<Vec<model::TodoItem>, anyhow::Error>),
    None,
}

impl Component for Todo {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        use model::TodoItem;
        Self {
            link,
            input: String::new(),
            show_error: false,
            list: vec![
                TodoItem {
                    id: 1,
                    text: "eat".to_owned(),
                },
                TodoItem {
                    id: 2,
                    text: "work".to_owned(),
                },
            ],
            user_name: "ishishow".to_owned(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInput(input) => {
                self.input = input;
                true
            }
            Msg::AddTodoItem => {
                if self.input.trim().len() == 0 {
                    self.show_error = true;
                } else {
                    self.list.push(model::TodoItem {
                        id: self.list.len() as i32,
                        text: self.input.clone(),
                    });
                }
                self.input = String::new();
                true
            }
            Msg::DeleteItem(id) => {
                self.list = self
                    .list
                    .clone()
                    .into_iter()
                    .filter(|item| item.id != id)
                    .collect();
                true
            }
            Msg::SetUserName(input) => {
                self.user_name = input;
                true
            }
            Msg::MakeReq => {
                self.list = None;
                let req = Request::get("http:localhost:8080/todos")
                    .body(Nothing)
                    .expect("can make req to jsonplaceholder");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<model::TodoItem>, anyhow::Error>>>| {
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
                    self.list = Some(data);
                }
            }

            _ => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let todos = self.list.clone();
        let cb = self.link.callback(|_| Msg::MakeReq);
        ConsoleService::info(&format!("render TodoApp: {:?}", todos));
        html! {
            <div class="ToDo">
            <img class="Logo" src={"./assets/rust.svg"} alt="Rust logo" />
            <img class="Logo" src={"./assets/yew.svg"} alt="Yew logo" />
            <h1 class="ToDo-Header">{self.user_name.clone()  + " ToDo!" }</h1>
            <div class="ToDo-Container">
              <div class="ToDo-Content">
                {todos
                  .iter()
                  .map(|item| html! {
                    <TodoItem
                      delete={self.link.callback(|id: i32| Msg::DeleteItem(id))}
                      item={item}
                    />
                  })
                  .collect::<Html>()}
              </div>

              <div class="ToDoInput">
                <input
                  type="text"
                  placeholder="I need to..."
                  value={&self.input}
                  oninput=self.link.callback(|evt: yew::events::InputData| Msg::UpdateInput(evt.value))
                  onkeypress=self.link.callback(|evt: yew::events::KeyboardEvent| if evt.key() == "Enter" { Msg::AddTodoItem } else { Msg::None })
                />
                <button
                  onclick=self.link.callback(|_| Msg::AddTodoItem)
                  class="ToDo-Add"
                >
                  {"+"}
                </button>
              </div>
              <div class="ToDo-ErrorContainer">{
                if self.show_error {
                  html! { <p>{ "Please enter a todo!" }</p> }
                } else {
                  html! {}
                }
              }</div>
            </div>
          </div>
        }
    }
}
