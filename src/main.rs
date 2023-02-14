use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::Serialize;
use std::env;

mod db;

#[derive(Serialize)]
struct Config {
    name: String,
    enabled: bool,
}

#[get("/version")]
async fn version() -> impl Responder {
    let version = env::var("FLAPPER_VERSION").unwrap_or_else(|_| "dev".to_string());
    HttpResponse::Ok().body(version)
}

// expose environment vars
async fn publish_envvars() -> Result<impl Responder> {
    let mut envvars = vec![];

    for (n, v) in env::vars() {
        // catch strings shorter than 2
        if n.len() >= 2 {
            let start = &n[..2];
            match start {
                "O_" => envvars.push(Config {
                    name: v.to_string(),
                    enabled: true,
                }),
                "X_" => envvars.push(Config {
                    name: v.to_string(),
                    enabled: false,
                }),
                &_ => (),
            }
        }
    }
    Ok(web::Json(envvars))
}

// expose db in a raw manner as dictionary
#[get("/db/raw")]
async fn publish_database() -> Result<impl Responder> {
    // call and unwrap db
    let databases = db::main();
    Ok(web::Json(databases))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    // read env var for path
    let prefix = env::var("PATH_PREFIX").unwrap_or_else(|_| "/".to_string());

    // print out some basic info about the server
    println!("Starting Flapper");
    println!("Serving at 0.0.0.0:8080{prefix}");

    // start server
    HttpServer::new(move || {
        App::new()
            .service(web::resource(&prefix).to(publish_envvars))
            .service(version)
            .service(publish_database)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
