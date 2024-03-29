use crate::api_keys::is_key_valid;
use crate::entities::parameter::Parameter;
use crate::AppState;
use actix_web::web::Data;
use actix_web::HttpRequest;
use sqlx::{MySql, Pool};
use std::sync::Mutex;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Fish {
    user_id: i64,
    tank_id: i64,
    fish_id: i64,
    name: String,
    species: String,
    qty: i64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FishRequest {
    user_id: i64,
    tank_id: i64,
    name: String,
    species: String,
    qty: i64,
}

// CREATE TABLE `fish` (
// `userid` INT NOT NULL,
// `tankid` INT NOT NULL,
// `fishid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `species` VARCHAR(255) NOT NULL,
// `qty` INT,
// PRIMARY KEY (`fishid`)
// ) ENGINE=InnoDB;

pub async fn create_fish_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
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

pub fn create_fish(user_id: i64, tank_id: i64, fish: Fish, db_pool: Pool<MySql>) {}

pub async fn delete_fish_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
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

pub fn delete_fish(user_id: i64, tank_id: i64, fish: Fish, db_pool: Pool<MySql>) {}

pub async fn modify_fish_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("X-API-KEY").is_some() {
        if is_key_valid(
            req.headers()
                .get("X-API-KEY")
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

pub fn modify_fish(user_id: i64, tank_id: i64, fish: Fish, db_pool: Pool<MySql>) {}
