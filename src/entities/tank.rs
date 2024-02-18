use crate::api_keys::is_key_valid;
use crate::entities::fish::Fish;
use crate::AppState;
use actix_web::web::Data;
use actix_web::HttpRequest;
use sqlx::{MySql, Pool};
use std::sync::Mutex;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Tank {
    user_id: i64,
    tank_id: i64,
    name: String,
    size_gallons: i64,
    height: i64,
    length: i64,
    width: i64,
    volume: i64,
    weight: f64,
    occupants: Vec<Fish>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TankRequest {
    user_id: i64,
    name: String,
    size_gallons: i64,
    height: i64,
    length: i64,
    width: i64,
    volume: i64,
    weight: f64,
    occupants: Vec<Fish>,
}

impl Tank {
    pub fn calc_volume(&mut self) {
        self.volume = (self.length / 12) * (self.width / 12) * (self.height / 12);
    }

    pub fn calc_approx_weight(&mut self) {
        self.calc_volume();
        self.weight = (self.volume as f64) * 7.47f64
    }
}

// CREATE TABLE `tank` (
// `userid` INT NOT NULL,
// `tankid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `size_gallons` INT NOT NULL,
// `height` INT,
// `length` INT,
// `width` INT,
// `volume` INT,
// `weight` FLOAT,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;

pub async fn create_tank_route(
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

pub fn create_tank(user_id: i64, tank: Tank, db_pool: Pool<MySql>) {}

pub async fn delete_tank_route(
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

pub fn delete_tank(user_id: i64, tank: Tank, db_pool: Pool<MySql>) {}

pub async fn modify_tank_route(
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

pub fn modify_tank(user_id: i64, tank: Tank, db_pool: Pool<MySql>) {}
