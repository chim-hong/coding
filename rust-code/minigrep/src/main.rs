use std::{env::args, process};

use minigrep::{Config, run};


fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("There is something wrong, the error is: {}",err);
        process::exit(0);
    });
    if let Err(e) = run(config){
        println!("Applycation is wrong! The error message is: {}",e);
        process::exit(0);
    }
}
