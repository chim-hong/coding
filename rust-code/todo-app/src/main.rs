use std::{
    io::stdin,
    process::{self},
};

use todo_app::{insert_todo_list, read_todo_list, ToDo};

/**
 * todo-app功能需求
 * 1. 用户通过命令行输入今日待办事项
 * 2. 用户可以通过 --ls命令来查看全部事项
 * 3. 用户可以通过 --done命令来查看已完成事项
 * 4. 用户可以通过 --undo命令查看未完成事项
 * 5. 程序内部维护一个db.txt用于存储全部的事项
 */
fn main() {
    println!("Welcome to todo app...");
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
        "todo --add\n"=>insert_todo_list(),
        _ => println!("wrong"),
    }


}
