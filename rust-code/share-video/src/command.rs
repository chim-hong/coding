use std::io::stdin;

use share_video::create_share_app;

fn welcome() {
    println!(
        "
        Welcome to share video app! Here is some commands you can use:
        -help --h:    get help
        -version --v: get version
        -list --l:    get list of videos platform
        Please type your command:
        "
    );
}
pub fn help() {
    welcome();
    let mut stop = false;
    while !stop {
        let mut command: String = String::new();
        stdin().read_line(&mut command).unwrap();
        println!("Your command is: {}", command.trim());

        match command.as_str().trim() {
            "-help" | "--h" => println!("You can type 'help' to get help"),
            "-version" | "--v" => println!("Current version is: {}", env!("CARGO_PKG_VERSION")),
            "-list" | "--l" => {
                println!(
                    "
                    Current support video platform is: IQyi, Youku, Tencent, Bilibili.
                    You can use command like `share Youku` to share your video on Youku.
                    "
                )
            }
            "-quit" | "-q" | "-exit" => {
                println!("Bye!");
                stop = true;
            }
            share_name if share_name.starts_with("share") => {
                let app_name = share_name.trim_start_matches("share").trim();
                create_share_app(app_name);
            }
            _ => println!("Your command is not found"),
        }
    }
}
