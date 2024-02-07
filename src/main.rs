// https://imfeld.dev/writing/actix-web-middleware
// curl -XGET -H'x-test-header: headervalue' localhost:8080/hello/asdf
mod api_keys;
mod middleware;

use middleware::*;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use crate::api_keys::{is_key_valid, load_keys_from_file};
use actix_files::{Files, NamedFile};
use actix_web::dev::Service;
use actix_web::http::header::HeaderMap;
use actix_web::http::{Method, StatusCode};
use actix_web::web::{service, Data};
use actix_web::{get, web, App, Either, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use config::Config;
use sqlx::{MySql, MySqlPool, Pool};

// #[get("/hello/{name}")]
async fn greet(
    name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    if is_key_valid(req.headers().get("x-test-header").unwrap().to_str().unwrap().to_string(), data.clone().lock().unwrap().api_key.lock().unwrap().to_vec()) {
        // verify api_key
        println!("{:#?}", data.clone().lock().unwrap().api_key);
        println!("{:#?}", data.clone().lock().unwrap().db_pool.lock().unwrap().is_closed());
        println!("{:#?}", req.headers());
        format!("Hello {name}!\n")
    } else {
        format!("invalid api key\n")
    }
}

pub struct AppState {
    api_key: Mutex<Vec<String>>,
    pub db_pool: Mutex<Pool<MySql>>,
}

impl AppState {
    pub fn new(keys: Vec<String>, db_pool: Pool<MySql>) -> AppState {
        AppState {
            api_key: Mutex::new(keys),
            db_pool: Mutex::new(db_pool)
        }
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(config::File::with_name("config/Settings"))
        .build()
        .expect("could not load Settings.toml");
    let settings_map = settings
        .try_deserialize::<HashMap<String, String>>()
        .expect("unable to deserialize settings");

    let database_url = settings_map.get("database_url").expect("could not get database_url from settings");

    // database connection
    let db_pool = MySqlPool::connect(database_url)
        .await
        .expect("unable to connect to database");

    // database connection state management

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(Mutex::new(AppState::new(load_keys_from_file(), db_pool.clone()))))
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
