use std::sync::{Mutex, MutexGuard};
use actix_web::{error, HttpRequest, HttpResponse, web};
use actix_web::error::ErrorBadRequest;
use actix_web::web::Data;
use futures_util::StreamExt;
use rand::{random, Rng};
use sqlx::{MySql, Pool, Row};
use crate::api_keys::is_key_valid;
use crate::AppState;
use crate::entities::tank::Tank;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct User {
    user_id: u16,
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
            if let Ok(obj) = serde_json::from_slice::<UserRequest>(&body) {
                let mut rand = rand::thread_rng();
                let new_user_id: u16 = rand.gen();

                let new_user = User {
                    user_id: new_user_id,
                    name: obj.name,
                    email: obj.email,
                    tanks: vec![],
                };

                println!("{:#?}", new_user.clone());
                let user_exists = check_user_exist(new_user.clone(), data.clone()).await;
                if !user_exists {
                    create_user(new_user.clone(), data.clone()).await;
                    "user created\n".to_string()
                } else if user_exists {
                    "user exists\n".to_string()
                } else {
                    "error creating user\n".to_string()
                }
            } else {
                "error creating user\n".to_string()
            }
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

async fn check_user_exist(user: User, mut data: Data<Mutex<AppState>>) -> bool {
    let mut user_exists: bool = false;
    // let data_2 = data.clone();
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

    // let mut app_state_2 = data_2.lock();
    // let mut db_pool_2 = app_state_2.as_mut().unwrap().db_pool.lock().unwrap();


    let mut query_result_string = String::new();
    if let Ok(query_result_1) = sqlx::query("SELECT userid FROM user WHERE userid=(?)")
        .bind(user.user_id)
        .fetch_one(&*db_pool)
        .await {
        query_result_string = query_result_1.get("userid");


    }

    let mut query_result_string_2 = String::new();
    if let Ok(query_result_2) = sqlx::query("SELECT email FROM user WHERE email LIKE (?)")
        .bind(user.email)
        .fetch_one(&*db_pool)
        .await {
        query_result_string = query_result_2.get("email");

    }


    if !query_result_string_2.is_empty() || !query_result_string.is_empty() {
        user_exists = true;
    }

    println!("User exists: {}", user_exists);
    user_exists
}

pub async fn create_user(user: User, data: Data<Mutex<AppState>>) {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();
    let is_closed = db_pool.is_closed();
    println!("Database connected: {}", !is_closed);

    let query_result = sqlx::query("INSERT INTO user (userid, name, email) VALUES (?,?,?)")
        .bind(user.user_id)
        .bind(user.name)
        .bind(user.email)
        .execute(&*db_pool)
        .await
        .unwrap();

    println!("{:#?}", query_result);
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
