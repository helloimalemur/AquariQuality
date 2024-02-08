use crate::api_keys::is_key_valid;
use crate::entities::tank::Tank;
use crate::AppState;
use actix_web::web::Data;
use actix_web::HttpRequest;
use sqlx::{MySql, Pool};
use std::sync::Mutex;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    user_id: i64,
    ph: i64,
    kh: i64,
}

// CREATE TABLE `parameter` (
// `userid` INT NOT NULL,
// `tankid` INT NOT NULL,
// `ph` INT,
// `kh` INT,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;

pub async fn create_parameter_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(
            req.headers()
                .get("x-api-key")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
        ) {
            "ok\n".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub fn create_parameter(user_id: i64, tank_id: i64, parameter: Parameter, db_pool: Pool<MySql>) {}

pub async fn delete_parameter_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(
            req.headers()
                .get("x-api-key")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
        ) {
            "ok\n".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub fn delete_parameter(user_id: i64, tank_id: i64, parameter: Parameter, db_pool: Pool<MySql>) {}

pub async fn modify_parameter_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(
            req.headers()
                .get("x-api-key")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            data.lock().unwrap().api_key.lock().unwrap().to_vec(),
        ) {
            "ok\n".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub fn modify_parameter(user_id: i64, tank_id: i64, parameter: Parameter, db_pool: Pool<MySql>) {}
