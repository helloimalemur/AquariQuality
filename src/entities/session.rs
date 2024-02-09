use crate::api_keys::is_key_valid;
use crate::entities::user::{create_user, User};
use crate::AppState;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{Data, Payload};
use actix_web::{error, web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use rand::{random, Rng};
use sqlx::{MySql, Pool, Row};
use std::sync::{Mutex, MutexGuard};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Session {
    user_id: u16,
    name: String,
    email: String,
    session_id: String
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct LoginRequest {
    user_id: u16,
    name: String,
    email: String,
    password: String,
}

// CREATE TABLE `session` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `sessionid` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`sessionid`)
// ) ENGINE=InnoDB;

pub async fn login_user_route(
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

            // body is loaded, now we can deserialize serde-json
            if let Ok(obj) = serde_json::from_slice::<crate::entities::user::UserRequest>(&body) {
                let mut rand = rand::thread_rng();
                let new_user_id: u16 = rand.gen();
                let user_req = obj.clone();
                let new_user = User {
                    user_id: new_user_id,
                    name: obj.name,
                    email: obj.email,
                    password: obj.password,
                    tanks: vec![],
                };

                println!("{:#?}", new_user.clone());
                let user_exists = crate::entities::user::check_user_exist(user_req, data.clone()).await;
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

pub async fn create_session(user: User, data: Data<Mutex<AppState>>) -> bool {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

    let new_session_id = generate_jwt_session_id(user.user_id).await;

    if let Ok(query_result) = sqlx::query("INSERT INTO session (userid,name,email,sessionid) VALUES (?,?,?,?)")
        .bind(user.user_id)
        .bind(user.name)
        .bind(user.email)
        .bind(new_session_id)
        .execute(&*db_pool)
        .await {
        true
    } else {
        false
    }
}

async fn generate_jwt_session_id(user_id: u16) -> String {
    let mut rand = rand::thread_rng();
    let temp_new_session_id: i64 = rand.gen();
    temp_new_session_id.to_string()
}

pub async fn delete_session(user: User, data: Data<Mutex<AppState>>) {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

}
pub async fn check_if_session_exists(user: User, data: Data<Mutex<AppState>>) {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

}
