use crate::api_keys::is_key_valid;
use crate::entities::session::{
    check_if_session_exists, check_if_session_exists_with_user_id, SessionId,
};
use crate::entities::tank::Tank;
use crate::AppState;
use actix_web::cookie::Expiration::Session;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{BytesMut, Data};
use actix_web::{web, HttpRequest};
use futures_util::StreamExt;
use sqlx::{MySql, Pool};
use std::sync::Mutex;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    user_id: i16,
    ph: f32,
    kh: f32,
    ammmonia: f32,
    nitrite: f32,
    nitrate: f32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ParameterRequest {
    session_id: String,
    user_id: i16,
    ph: f32,
    kh: f32,
    ammmonia: f32,
    nitrite: f32,
    nitrate: f32,
}

// CREATE TABLE `parameter` (
// `userid` INT NOT NULL,
// `ph` FLOAT,
// `kh` FLOAT,
// `ammonia` FLOAT,
// `nitrite` FLOAT,
// `nitrate` FLOAT,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;

// curl -XPOST -H'x-api-key: omganotherone' localhost:8080/api/create/parameter/ -d '{ "session_id": "String", "user_id": 4412, "ph": 0.0, "kh": 0.0, "ammmonia": 0.0, "nitrite": 0.0, "nitrate": 0.0}'
pub async fn create_parameter_route(
    // name: web::Path<String>,
    mut payload: web::Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k

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
            let mut body = web::BytesMut::new();

            while let Some(chunk) = payload.next().await {
                let chunk = chunk.unwrap();
                // limit max size of in-memory payload
                if (body.len() + chunk.len()) > MAX_SIZE {
                    return ErrorBadRequest("overflow").to_string();
                }
                body.extend_from_slice(&chunk);
            }

            let param_request = serde_json::from_slice::<ParameterRequest>(&body).unwrap();
            let param = Parameter {
                user_id: param_request.user_id,
                ph: param_request.ph,
                kh: param_request.kh,
                ammmonia: param_request.ammmonia,
                nitrite: param_request.nitrite,
                nitrate: param_request.nitrate,
            };

            // let session_exists = check_if_session_exists(SessionId::new(param_request.session_id), db_pool.clone()).await;
            let session_exists =
                check_if_session_exists(SessionId::new(param_request.session_id), data.clone())
                    .await;

            println!("{}", session_exists);

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
