use crate::components::todo::Item;
// use crate::model;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct TodoItemProps {
    pub item: Item,
    pub delete: Callback<i32>,
}

pub struct TodoItem {
    link: ComponentLink<Self>,
    props: TodoItemProps,
}

pub enum Msg {
    OnClick,
}

impl Component for TodoItem {
    type Message = Msg;
    type Properties = TodoItemProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnClick => {
                let id = self.props.item.id.clone();
                self.props.delete.emit(id);
                return false;
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        html! {
            <div class="ToDoItem">
                <p class="ToDoItem-Text">{&self.props.item.title}</p>
                <button
                    onclick={self.link.callback(|_| Msg::OnClick)}
                    class="ToDoItem-Delete"
                >
                { "-" }
                </button>
            </div>
        }
    }
}
