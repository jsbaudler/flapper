use actix_web::{web, Responder, Result};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Config {
    name: String,
    enabled: bool,
}

// create the JSON response
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

    // read env var for path
    let prefix = env::var("PATH_PREFIX").unwrap_or("/".to_string());

    // print out some basic info about the server
    println!("Starting Flapper");
    println!("Serving at prefix: 0.0.0.0:8080{}", prefix);

    // start server
    HttpServer::new(move || App::new().service(web::resource(&prefix).to(index)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
