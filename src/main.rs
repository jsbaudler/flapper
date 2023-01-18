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

    for (n,v) in env::vars() {
        if n.starts_with("O_") {
            envvars.push(Config{name: v.to_string(), enabled: true});
        } else if n.starts_with("X_") {
            envvars.push(Config{name: v.to_string(), enabled: false});
        };
    };
    Ok(web::Json(envvars))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    println!("Starting Flapper");
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
