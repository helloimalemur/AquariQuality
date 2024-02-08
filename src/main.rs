// https://imfeld.dev/writing/actix-web-middleware
// curl -XGET -H'x-api-key: headervalue' localhost:8080/hello/asdf
mod api_keys;
mod middleware;
mod entities;

use crate::api_keys::{create_api_key, delete_api_key, is_key_valid, load_keys_from_file};
use crate::middleware::api_key;
use std::collections::HashMap;
use std::sync::Mutex;
use actix_files::{NamedFile};
use actix_web::dev::Service;
use actix_web::http::{Method, StatusCode};
use actix_web::web::{Data};
use actix_web::{get, web, App, Either, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use config::Config;
use sqlx::{MySql, MySqlPool, Pool};
use crate::entities::parameter::{create_parameter_route, delete_parameter_route, modify_parameter_route};
use crate::entities::tank::{create_tank_route, delete_tank_route, modify_tank_route};
use crate::entities::user::{create_user_route, delete_user_route, modify_user_route};

async fn root(
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    if is_key_valid(req.headers().get("x-api-key").unwrap().to_str().unwrap().to_string(), data.clone().lock().unwrap().api_key.lock().unwrap().to_vec()) {
        "Hello Astronaut!\n".to_string()
    } else {
        "invalid api key\n".to_string()
    }
}


pub struct AppState {
    api_key: Mutex<Vec<String>>,
    db_pool: Mutex<Pool<MySql>>,
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

    let state = Data::new(Mutex::new(AppState::new(load_keys_from_file(), db_pool.clone())));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(api_key::ApiKey::new("".to_string()))
            // src/api_keys
            .service(web::resource("/api/create/key/").to(create_api_key))
            .service(web::resource("/api/delete/{key}").to(delete_api_key))
            .service(web::resource("/api/delete/{key}/").to(delete_api_key))
            // src/entities/users
            .service(web::resource("/api/create/user/").post(create_user_route))
            .service(web::resource("/api/delete/user/").post(delete_user_route))
            .service(web::resource("/api/modify/user/").post(modify_user_route))
            // src/entities/tanks
            .service(web::resource("/api/create/tank/").post(create_tank_route))
            .service(web::resource("/api/delete/tank/").post(delete_tank_route))
            .service(web::resource("/api/modify/tank/").post(modify_tank_route))
            // src/entities/parameters
            .service(web::resource("/api/create/parameter/").post(create_parameter_route))
            .service(web::resource("/api/delete/parameter/").post(delete_parameter_route))
            .service(web::resource("/api/modify/parameter/").post(modify_parameter_route))
            //
            .service(web::resource("/").to(root))
            .default_service(web::to(default_handler))
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
