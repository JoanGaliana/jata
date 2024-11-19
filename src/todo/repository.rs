use std::collections::HashMap;
use crate::todo::model::Todo;
use std::sync::{LazyLock, Mutex};
use uuid::Uuid;

static TODOS: LazyLock<Mutex<HashMap<Uuid,Todo>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub struct TodoRepository {}

impl TodoRepository {
    pub fn get_all() -> Vec<Todo> {
        let todos = TODOS.lock().unwrap();

        todos.values().map(Todo::clone).collect()
    }

    pub fn create(todo: &Todo) -> Result<(), ()> {
        let mut todos = TODOS.lock().unwrap();

        todos.insert(todo.id, todo.clone());

        Ok(())
    }

    pub fn modify(todo: &Todo) -> Result<(), String> {
        let mut todos = TODOS.lock().unwrap();

       if !todos.contains_key(&todo.id){
           return Err("Not found".to_string());
       }

        todos.insert(todo.id, todo.clone());

        Ok(())
    }
}
