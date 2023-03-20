use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub todo_content: String,
    pub is_done: bool,
}

impl Todo {
    pub fn new(message: &String) -> Todo {
        Todo {
            todo_content: message.trim_end().to_owned(),
            is_done: false,
        }
    }
}
