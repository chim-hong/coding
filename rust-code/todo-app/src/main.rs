use std::{ process};

use todo_app::{command_input, create_or_open_file};

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
    println!("Check db file...");
    match create_or_open_file() {
        Ok(_) => {
            println!("DB file is ready!");
            command_input();
        }
        Err(_) => {
            println!("Something wrong when init db text!");
            process::exit(0);
        }
    }
}
