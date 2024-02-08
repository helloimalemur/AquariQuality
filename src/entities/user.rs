use std::sync::{Mutex, MutexGuard};
use actix_web::{error, HttpRequest, HttpResponse, web};
use actix_web::error::ErrorBadRequest;
use actix_web::web::Data;
use futures_util::StreamExt;
use rand::{random, Rng};
use sqlx::{MySql, Pool};
use crate::api_keys::is_key_valid;
use crate::AppState;
use crate::entities::tank::Tank;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct User {
    user_id: i64,
    name: String,
    email: String,
    tanks: Vec<Tank>
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct UserRequest {
    name: String,
    email: String
}

// CREATE TABLE `user` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;

// curl -XPOST -H'x-api-key: 12790066417744034365' localhost:8080/api/create/user/ -d '{"name":"johnny","email":"johhny@mail.com"}'
pub async fn create_user_route(
    // name: web::Path<String>,
    mut payload: web::Payload,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    const MAX_SIZE: usize = 262_144; // max payload size is 256k
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(req.headers().get("x-api-key").unwrap().to_str().unwrap().to_string(), data.lock().unwrap().api_key.lock().unwrap().to_vec()) {

            let mut body = web::BytesMut::new();

            while let Some(chunk) = payload.next().await {
                let chunk = chunk.unwrap();
                // limit max size of in-memory payload
                if (body.len() + chunk.len()) > MAX_SIZE {
                    return ErrorBadRequest("overflow").to_string();
                }
                body.extend_from_slice(&chunk);
            }

            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<UserRequest>(&body).unwrap();
            let mut rand = rand::thread_rng();
            let new_user_id: i64 = rand.gen();

            let new_user = User {
                user_id: new_user_id,
                name: obj.name,
                email: obj.email,
                tanks: vec![],
            };

            println!("{:#?}", new_user.clone());



            create_user(new_user, data.clone());

            "ok\n".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub fn create_user(user: User, data: Data<Mutex<AppState>>) {
    let mut binding = data.lock();
    let mut app_state = binding.as_mut().unwrap().db_pool.lock();
    let is_closed = app_state.unwrap().is_closed();
    println!("{}", !is_closed);
}

pub async fn delete_user_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(req.headers().get("x-api-key").unwrap().to_str().unwrap().to_string(), data.lock().unwrap().api_key.lock().unwrap().to_vec()) {
            "ok\n".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub fn delete_user(user: User, db_pool: Pool<MySql>) {}

pub async fn modify_user_route(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(req.headers().get("x-api-key").unwrap().to_str().unwrap().to_string(), data.lock().unwrap().api_key.lock().unwrap().to_vec()) {
            "ok\n".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub fn modify_user(user: User, db_pool: Pool<MySql>) {}
