// https://imfeld.dev/writing/actix-web-middleware
// curl -XGET -H'x-test-header: headervalue' localhost:8080/hello/asdf
mod middleware;
mod api_keys;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use middleware::*;

use actix_web::{get, web, App, HttpServer, Responder, Either, HttpResponse, Error};
use actix_web::http::{Method, StatusCode};
use actix_files::{Files, NamedFile};
use actix_web::dev::Service;
use actix_web::web::{Data, service};
use config::Config;
use crate::api_keys::load_keys_from_file;

struct AppState {
    api_keys: Mutex<Vec<String>>
}

// #[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!\n")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/Settings"))
        .build()
        .expect("could not load Settings.toml");
    let settings_map = settings.try_deserialize::<HashMap<String, String>>()
        .expect("unable to deserialize settings");


    // Note: web::Data created _outside_ HttpServer::new closure
    let app_state = web::Data::new(AppState {
        api_keys: Mutex::new(load_keys_from_file())
    });

    // struct AppStateWithCounter {
    //     counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
    // }
    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_state.clone()))
            .wrap(middleware::api_key::ApiKey::new("asdf".to_string()))

            // .service(root)
            // .service()
            .default_service(web::to(default_handler))
            .service(web::resource("/hello/{name}").to(greet))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


async fn default_handler(req_method: Method) -> Result<impl Responder, Error> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        }
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())),
    }
}
