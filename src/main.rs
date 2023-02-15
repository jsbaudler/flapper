use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct Config {
    name: String,
    enabled: bool,
}

// expose version and flapper version
#[get("/version")]
async fn version() -> Result<impl Responder> {

    // env vars
    let flapper_version =
        env::var("FLAPPER_VERSION").unwrap_or_else(|_| "0.0.0-dev (not set)".to_string());
    let version_file_path = env::var("VERSION_PATH").unwrap_or_else(|_| "example.json".to_string());

    // prepare flapper version
    let flapper_version = serde_json::to_value(flapper_version).unwrap();
    let mut flapper_version: HashMap<String, Value> =
        HashMap::from([("flapper_version".to_string(), flapper_version)]);

    if Path::new(&version_file_path).exists() {

        // read and consume version file
        let version_file = fs::read_to_string(version_file_path).unwrap();
        serde_json::to_string(&version_file).unwrap();
        let version: HashMap<String, Value> = serde_json::from_str(&version_file).unwrap();

        // combine
        flapper_version.extend(version);
    } else {
        println!("Version File not Found: {}", version_file_path)
    }



    Ok(web::Json(flapper_version))
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
