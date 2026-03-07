use std::sync::Arc;
use chrono::Utc;
use dashmap::DashMap;
use tokio::task::id;
use uuid;
use uuid::Uuid;
use crate::modules::{Todo, TodoResponse};

pub struct Db {
    pub todos: Arc<DashMap<Uuid, Arc<Todo>>>
}
impl Db {
    pub fn new() -> Self {
        Self{todos: Arc::new(DashMap::new())}
    }
    pub fn create(&self, title: String) -> Arc<Todo> {
        let todo = Arc::new(Todo{
            id: Uuid::new_v4(),
            title,
            created_at: Utc::now(),
            completed: false,
        });

        self.todos.insert(todo.id, Arc::clone(&todo));
        todo
    }
    pub fn update(&self, id: Uuid, title:Option<String>, completed:Option<bool>) -> bool{
        if let Some(mut entry) = self.todos.get_mut(&id) {
            let todo_mut = Arc::make_mut(&mut entry);
            if let Some(completed) = completed {
                todo_mut.completed = completed;
            }
            if let Some(title) = title {
                todo_mut.title = title;
            }
            true
        } else {
            false
        }
    }
    pub fn delete(&self, id: Uuid) -> bool {
        self.todos.remove(&id).is_some()
    }
    pub fn get(&self, id: Uuid) -> Option<Todo> {
        let todo = self.todos.get(&id).map(|entry| (**entry).clone());
        todo
    }
    pub fn get_all(&self) -> Vec<Todo>{
        self.todos.iter().map(|x| {
            let todo = (**x).clone();
            todo
        }).collect()
    }
}