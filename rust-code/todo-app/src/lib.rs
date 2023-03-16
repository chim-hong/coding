use std::{
    fs::{self, File},
    io::{stdin, stdout},
    process,
};

pub struct ToDo<'a> {
    pub create_time: &'a str,
    pub done_time: &'a str,
    pub message: &'a str,
    pub is_done: bool,
}

impl<'a> ToDo<'a> {
    pub fn new(todo: &String) -> ToDo {
        ToDo {
            create_time: "2020-03-13",
            message: todo,
            done_time: "2020-03-13",
            is_done: false,
        }
    }
}

fn loop_insert(db_path: &str) {
    let mut todo_message = String::new();
    loop {
        stdin()
            .read_line(&mut todo_message)
            .expect("something went wrong when type message!");
        // 原先的todo数据
        let contents = fs::read_to_string(db_path).unwrap_or_else(|error| {
            println!("something wrong :{:#?}", error);
            process::exit(0);
        });
        println!("todo list:{:#?}", contents);
        let new_contents = format!("{} \n {}", contents, todo_message);
        fs::write(db_path, new_contents).unwrap_or_else(|error| {
            println!("something wrong :{:#?}", error);
        });
        if todo_message.as_str() == "exit" {
            println!("222")
        }
    }
}

pub fn insert_todo_list() {
    let db_path = "./db.txt";
    // 判断是否存在db.txt文件，不存在则创建文件
    match File::open(db_path) {
        Ok(_) => loop_insert(db_path),
        Err(_) => {
            println!("There is no such file, we'll create a file which store the todo list!");
            match File::create(db_path) {
                Ok(_) => println!("db has been create!"),
                Err(_) => println!("something wrong when create db"),
            }
            println!("22323");
        }
    };
}

pub fn read_todo_list(mode: &str) {
    let todo_list = fs::read_to_string("./db.txt");
    let todo_list = match todo_list {
        Ok(todos) => todos,
        Err(_) => "something wrong".to_string(),
    };
    let todo_collect: Vec<&str> = todo_list.split("\n").collect();
    for todo in todo_collect {
        if !todo.is_empty() {
            let todo_item: Vec<&str> = todo.split("|").collect();
            if mode == "all" {
                println!(" | {} | {} | {}", todo_item[0], todo_item[1], todo_item[2]);
            }
            if mode == "done" {
                if todo_item[2] == "is done" || todo_item[2] == "true" {
                    println!(" | {} | {} | {}", todo_item[0], todo_item[1], todo_item[2])
                }
            }
            if mode == "undo" {
                if todo_item[2] == "is done" || todo_item[2] == "false" {
                    println!(" | {} | {} | {}", todo_item[0], todo_item[1], todo_item[2])
                }
            }
        }
    }
}

#[cfg(test)]
mod test_todo_app {
    use super::*;

    #[test]
    fn test_read_all_todo_list() {
        read_todo_list("all");
    }
    fn test_read_done_todo_list() {
        read_todo_list("done");
    }
    fn test_read_undo_todo_list() {
        read_todo_list("done");
    }
}
