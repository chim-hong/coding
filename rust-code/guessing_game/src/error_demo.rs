use std::{
    fs::File,
    io::{self, Error, Read, Write},
};

pub fn open_and_read_file() -> Result<String, Error> {
    let mut contents = String::new();
    File::open("./src/hello.txt")?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn create_wirte_file() -> Result<String, Error> {
    println!("we found that there's no file named 'hello.txt', so we'll create it! ");
    let mut create_file = File::create("./src/hello.txt")?;
    create_file.write("hello, world!".as_bytes())?;
    custom_write_file();
    Ok("create and write success!".to_string())
}

fn custom_write_file() {
    println!("Now you can type something you want append to the file witch created just now!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("something error!");
    let mut old_str = String::new();

    let new_str = old_str + &input;

    println!("{:?},{:?}", input,new_str);
}
