// https://imfeld.dev/writing/actix-web-middleware
// curl -XGET -H'X-API-KEY: headervalue' localhost:8723/hello/asdf
mod api_keys;
mod entities;
mod frontend;
mod middleware;

use crate::api_keys::{create_api_key, delete_api_key, is_key_valid, load_keys_from_file};
use crate::entities::parameter::{
    create_parameter_route, delete_parameter_route, modify_parameter_route,
};
use crate::entities::session::{login_user_route, logout_user_route};
use crate::entities::tank::{create_tank_route, delete_tank_route, modify_tank_route};
use crate::entities::user::{create_user_route, delete_user_route, modify_user_route};
use crate::frontend::start_front_end;
use crate::middleware::api_key;
use actix_files::NamedFile;
use actix_web::dev::Service;
use actix_web::http::{Method, StatusCode};
use actix_web::web::Data;
use actix_web::{get, web, App, Either, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use config::Config;
use sqlx::{MySql, MySqlPool, Pool};
use std::collections::HashMap;
use std::sync::Mutex;
use actix_cors::Cors;

async fn root(data: Data<Mutex<AppState>>, req: HttpRequest) -> String {
    if is_key_valid(
        match req.headers()
            .get("X-API-KEY") {
            Some(x) => {
               x.to_str()
                    .unwrap()
                    .to_string()
            },
            None => "".to_string(),
        },
        data.clone()
            .lock()
            .unwrap()
            .api_key
            .lock()
            .unwrap()
            .to_vec(),
    ) {
        "Hello Astronaut!\n".to_string()
    } else {
        "invalid api key\n".to_string()
    }
}

pub struct AppState {
    api_key: Mutex<Vec<String>>,
    db_pool: Mutex<Pool<MySql>>,
    settings: Mutex<HashMap<String, String>>,
}

impl AppState {
    pub fn new(
        keys: Vec<String>,
        db_pool: Pool<MySql>,
        settings_map: HashMap<String, String>,
    ) -> AppState {
        AppState {
            api_key: Mutex::new(keys),
            db_pool: Mutex::new(db_pool),
            settings: Mutex::new(settings_map),
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

    let http_service_port = settings_map
        .get("http_service_port")
        .expect("could not get http_service_port from settings")
        .parse::<u16>()
        .unwrap();

    let database_url = settings_map
        .get("database_url")
        .expect("could not get database_url from settings");

    let frontend_enabled = settings_map
        .get("frontend_enabled")
        .expect("could not get frontend_enabled from settings")
        .parse::<bool>()
        .expect("could not get frontend_enabled from settings");

    println!("Connecting to Database...");
    // database connection
    let db_pool = MySqlPool::connect(database_url)
        .await
        .expect("unable to connect to database");

    if !db_pool.is_closed() && frontend_enabled {
        let _ = start_front_end().await;
    }

    if !db_pool.is_closed() {
        println!("Successfully Connected");
    }

    let state = Data::new(Mutex::new(AppState::new(
        load_keys_from_file(),
        db_pool.clone(),
        settings_map.clone(),
    )));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            // .allow_any_method()
            .allowed_methods(vec!["GET", "POST", "OPTIONS", "X-API-KEY"])
            // .allowed_origin("*")
            // .allowed_methods(["OPTIONS","GET", "POST"])
            // .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .wrap(api_key::ApiKey::new("".to_string()))
            // login/logout
            .service(web::resource("/login/").post(login_user_route))
            .service(web::resource("/logout/").post(logout_user_route)) // todo
            // src/api_keys
            .service(web::resource("/api/create/").post(create_api_key))
            .service(web::resource("/api/delete/").post(delete_api_key))
            // src/entities/users
            .service(web::resource("/user/create/").post(create_user_route))
            .service(web::resource("/user/delete/").post(delete_user_route))
            // .service(web::resource("/api/modify/user/").post(modify_user_route))
            // // src/entities/tanks
            // .service(web::resource("/api/create/tank/").post(create_tank_route))
            // .service(web::resource("/api/delete/tank/").post(delete_tank_route))
            // .service(web::resource("/api/modify/tank/").post(modify_tank_route))
            // // src/entities/parameters
            .service(web::resource("/api/create/parameter/").post(create_parameter_route))
            // .service(web::resource("/api/delete/parameter/").post(delete_parameter_route))
            // .service(web::resource("/api/modify/parameter/").post(modify_parameter_route))
            //
            .service(web::resource("/").to(root))
            .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", http_service_port))?
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
