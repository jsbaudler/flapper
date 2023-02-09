use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Config {
    name: String,
    enabled: bool,
}

#[get("/version")]
async fn version() -> impl Responder {
    let version = env::var("FLAPPER_VERSION").unwrap_or("dev".to_string());
    HttpResponse::Ok().body(version.to_string())
}

// create the JSON response
async fn publish_envvars() -> Result<impl Responder> {
    let mut envvars = vec![];

    for (n,v) in env::vars() {
        
        let start = &n[..2];

        match start {
            "O_" => envvars.push(Config{name: v.to_string(), enabled: true}),
            "X_" => envvars.push(Config{name: v.to_string(), enabled: false}),
            &_ => (),
        }

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
    println!("Serving at 0.0.0.0:8080{prefix}");

    // start server
    HttpServer::new(move || {
        App::new()
        .service(web::resource(&prefix).to(publish_envvars))
        .service(version)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}