use actix_web::{middleware::Logger, web, App, HttpServer, Responder, Result};
use env_logger::Env;
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
async fn publish_version() -> Result<impl Responder> {
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
        let version_file = fs::read_to_string(&version_file_path).map_err(|e| {
            log::error!("Failed to read version file: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to read version file")
        })?;
        let version: HashMap<String, Value> = serde_json::from_str(&version_file).map_err(|e| {
            log::error!("Failed to parse version file: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to parse version file")
        })?;

        // combine
        flapper_version.extend(version);
    } else {
        log::warn!("Version File not Found: {version_file_path}");
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
    // initialize logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // read env var for path
    let env_var_prefix: String = env::var("ENV_VAR_PREFIX").unwrap_or_else(|_| "/env".to_string());
    let version_prefix: String =
        env::var("VERSION_PREFIX").unwrap_or_else(|_| "/version".to_string());

    // check that env_var_prefix and version_prefix are not equal
    if env_var_prefix == version_prefix {
        panic!("ENV_VAR_PREFIX and VERSION_PREFIX cannot be the same.");
    }

    let port_str: String = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let port = match port_str.parse::<u16>() {
        Ok(port) if (1..=65535).contains(&port) => port,
        _ => {
            panic!(
                "Invalid port number specified in SERVER_PORT environment variable: {}",
                port_str
            );
        }
    };

    // print out some basic info about the server
    log::info!("Starting Flapper");
    log::info!("Serving at 0.0.0.0:{port}");
    log::info!("Serving environment variables at {env_var_prefix}");
    log::info!("Serving version at {version_prefix}");

    // start server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::resource(&env_var_prefix).to(publish_envvars))
            .service(web::resource(&version_prefix).to(publish_version))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    // test if the env endpoint works
    #[actix_web::test]
    async fn test_envvars() {
        env::set_var("O_VARIABLE_1", "bird");
        env::set_var("X_VARIABLE_2", "dolphin");

        let mut app =
            test::init_service(App::new().service(web::resource("/env").to(publish_envvars))).await;

        let req = test::TestRequest::get().uri("/env").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        println!("{body_str}");
        assert!(body_str.contains(
            "[{\"name\":\"bird\",\"enabled\":true},{\"name\":\"dolphin\",\"enabled\":false}]"
        ));
    }

    // test if the version endpoint works
    #[actix_web::test]
    async fn test_version() {
        let mut app =
            test::init_service(App::new().service(web::resource("/version").to(publish_version)))
                .await;

        let req = test::TestRequest::get().uri("/version").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains("flapper_version"));
    }

    // test if the server starts and both endpoints work together
    #[actix_web::test]
    async fn test_server_startup() {
        env::set_var("O_VARIABLE_1", "bird");
        env::set_var("X_VARIABLE_2", "dolphin");
        let mut app = test::init_service(
            App::new()
                .service(web::resource("/env").to(publish_envvars))
                .service(web::resource("/version").to(publish_version)),
        )
        .await;

        let req = test::TestRequest::get().uri("/env").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains(
            "[{\"name\":\"bird\",\"enabled\":true},{\"name\":\"dolphin\",\"enabled\":false}]"
        ));

        let req = test::TestRequest::get().uri("/version").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains("flapper_version"));
    }
}

