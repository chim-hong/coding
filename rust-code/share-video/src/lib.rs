use std::io::stdin;

use futures::executor::block_on;
use reqwest::Client;

/// This is a function to accomplish share video app
pub fn create_share_app(app_name: &str) {
    println!("The app you shared is {}", app_name);
    println!("Please type your account:");
    let mut account = String::new();
    stdin().read_line(&mut account).unwrap();
    println!("Please type your password:");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    println!(
        "Your account is: {}, password is: {}.",
        account.trim(),
        password.trim()
    );
    let info = block_on(get_app_login_info());
    match info {
        Ok(_) => println!("Get login info success"),
        Err(e) => println!("Get login info failed, error is: {}", e),
    }
    // Todo: add share video app logic
}

async fn get_app_login_info() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get("https://baidu.com").send().await?;
    println!("{:#?}", response.text().await?);
    Ok(())
}
