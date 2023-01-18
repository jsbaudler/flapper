use actix_web::{get, web, Responder, Result};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Config {
    name: String,
    enabled: bool,
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    let mut envvars = vec![];

    for (n,_) in env::vars() {
        if n.starts_with("XDG_") {
            let (_, value) = n.split_once("_").unwrap();
            envvars.push(Config{name: value.to_string(), enabled: true});
        };
    };

    Ok(web::Json(envvars))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    println!("HUBBABUBBA GOES UUUUP");

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
