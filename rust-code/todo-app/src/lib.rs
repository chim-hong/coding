use chrono::Local;
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::stdin,
};

mod todo;

fn loop_insert() {
    let mut todo_message = String::new();
    loop {
        stdin()
            .read_line(&mut todo_message)
            .expect("something went wrong when type message!");
        insert_todo_list(&todo_message);
    }
}

pub fn insert_todo_list(message: &String) {
    let db_path = env::var("HOME").unwrap() + "/Desktop/db.json";

    let fmt = "%Y-%m-%d";
    let today = Local::now().format(fmt).to_string();
    println!("today:{}", today);

    // get todo's data
    let todo_list = match fs::read_to_string(&db_path) {
        Ok(it) => it,
        Err(_) => String::new(),
    };

    // json to object
    let mut todos: HashMap<String, Vec<todo::Todo>> = serde_json::from_str(&todo_list).unwrap();
    let date_vec = todos.keys().any(|date| date == &today);
    let new_todo: todo::Todo = todo::Todo::new(&message);
    if date_vec {
        // 向今日插入数据
        let today_todo = todos.get_mut(&today).unwrap();
        today_todo.push(new_todo);
    } else {
        todos.insert(today, vec![new_todo]);
    }
    // 写入文件
    let contents: String = serde_json::to_string(&todos).unwrap();
    fs::write(&db_path, contents).unwrap();
}

pub fn read_todo_list(mode: &str) {
    println!("mode:{:#?}", mode);

    let db_path = "./db.json";
    // get todo's data
    let todo_list = match fs::read_to_string(db_path) {
        Ok(it) => it,
        Err(_) => String::new(),
    };

    // json to object
    let todos: HashMap<String, Vec<todo::Todo>> = serde_json::from_str(&todo_list).unwrap();
    // get all todo
    for date in todos.keys() {
        println!("date: {}", date);
        for todo in &todos[date] {
            if mode == "all" {
                println!("{}-{}", todo.todo_content, todo.is_done)
            }
            if mode == "done" && todo.is_done {
                println!("{}-{}", todo.todo_content, todo.is_done)
            }
        }
    }
}

// 打开文件
pub fn create_or_open_file() -> Result<File, Box<dyn Error>> {
    let db_path = "./db.json";
    let file = match File::open(db_path) {
        Ok(file) => {
            println!("db text has already!");
            file
        }
        Err(_) => {
            println!("There's no db file, prepare to create the file");
            File::create(db_path)?
        }
    };
    Ok(file)
}

// 命令输入
pub fn command_input() {
    println!(
        "
    There is some command to use:
    1. todo --ls 查看全部事项
    2. todo --done 查看已完成事项
    3. todo --undo 查看未完成事项

    Now please entry a command...
    "
    );
    let mut command = String::new();
    stdin()
        .read_line(&mut command)
        .expect("Something went wrong!");
    match &command as &str {
        "todo --ls\n" => read_todo_list("all"),
        "todo --done\n" => read_todo_list("done"),
        "todo --undo\n" => read_todo_list("undo"),
        "todo --add\n" => loop_insert(),
        _ => println!("wrong"),
    }
}

#[cfg(test)]
mod test_todo_app {
    use super::*;

    #[test]
    fn test_read_all_todo_list() {
        read_todo_list("all");
    }
    #[test]
    fn test_read_done_todo_list() {
        read_todo_list("done");
    }
    #[test]
    fn test_read_undo_todo_list() {
        read_todo_list("undo");
    }
}
