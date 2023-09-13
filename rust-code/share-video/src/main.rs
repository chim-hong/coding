use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
mod fs;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct LoginParams {
    account: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Response {
    code: i32,
    msg: String,
    data: Value,
}

#[post("/login")]
async fn login(params: web::Json<LoginParams>) -> impl Responder {
    let json = json!(params);
    let mut flag = false;

    let res: Response = if params.account == "admin" && params.password == "123456" {
        println!("Login success");
        Response {
            code: 200,
            msg: "success".to_string(),
            data: json.clone(),
        }
    } else {
        println!("Login failed");
        let msg = String::from("password or account is wrong");
        flag = true;
        Response {
            code: 201,
            msg: "failed".to_string(),
            data: json!(msg),
        }
    };
    let res = serde_json::to_value(&res).unwrap();
    println!("res is: {:#?}", res);
    if flag  {
        HttpResponse::Ok().body(fs::read_404())
    }else {
        HttpResponse::Ok().body(json!(res).to_string())
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(login).route(
            "/",
            web::get().to(|| async { HttpResponse::Ok().body(fs::read_index()) }),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
