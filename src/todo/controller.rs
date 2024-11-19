use crate::todo::model::Todo;
use crate::todo::repository::TodoRepository;
use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum::Form;

pub const TODO_BASE_PATH: &'static str = "/todo";
pub const TODO_CREATE_PATH: &'static str = "/todo/create";
pub const TODO_MODIFY_PATH: &'static str = "/todo/modify";

#[tracing::instrument]
async fn get_all() -> impl IntoResponse {
    let todos = TodoRepository::get_all();
    let template = TodosTableTemplate { todos };
    HtmlTemplate(template)
}

#[tracing::instrument]
async fn create(Form(todo): Form<Todo>) -> Redirect {
    let message = TodoRepository::create(&todo).map_or("Error creating todo", |_| "Successfully");

    Redirect::to(&format!("{TODO_BASE_PATH}?message={message}"))
}

#[tracing::instrument]
async fn modify(Form(todo): Form<Todo>) -> Redirect {
    let message = TodoRepository::modify(&todo)
        .map_or_else(|err|format!("Error: {err}"), |_| " Successfully".to_string());

    Redirect::to(&format!("{TODO_BASE_PATH}?message={message}"))
}

#[tracing::instrument]
pub fn get_router() -> axum::Router {
    axum::Router::new()
        .route(TODO_BASE_PATH, get(get_all))
        .route(TODO_CREATE_PATH, post(create))
        .route(TODO_MODIFY_PATH, post(modify))

}

#[derive(Template)]
#[template(path = "todos_table.html")]
struct TodosTableTemplate {
    todos: Vec<Todo>,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
