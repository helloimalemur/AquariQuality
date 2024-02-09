use crate::api_keys::is_key_valid;
use crate::entities::user::{create_user, User, UserRequest};
use crate::AppState;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{Data, Payload};
use actix_web::{error, web, HttpRequest, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};
use rand::{random, Rng};
use sqlx::{Error, MySql, Pool, Row};
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
            if let Ok(obj) = serde_json::from_slice::<LoginRequest>(&body) {

                let login_req = obj.clone();
                let login_request = LoginRequest {
                    email: obj.email,
                    password: obj.password,
                };

                println!("{:#?}", login_request.clone());
                let user_exists = crate::entities::user::check_user_exist(login_req.email, data.clone()).await;

                if user_exists {
                    // process login

                    create_session(login_request, data.clone()).await;

                    "user login successful\n".to_string()
                } else if !user_exists {
                    "user does not exist\n".to_string()
                } else {
                    "error logging in\n".to_string()
                }
            } else {
                "error logging in\n".to_string()
            }
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub async fn create_session(user_login_request: LoginRequest, data: Data<Mutex<AppState>>) -> bool {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

    let user: User = get_user_from_login_request(user_login_request, data.clone()).await;

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

async fn get_user_from_login_request(user_login_request: LoginRequest, data: Data<Mutex<AppState>>) -> User {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();
    // let query_result =
    let mut rows = sqlx::query("SELECT * FROM user WHERE email LIKE (?) AND password LIKE (?)")
        .bind(user_login_request.email)
        .bind(user_login_request.password)
        .fetch(&*db_pool);

    let mut user = User {
        user_id: 0,
        name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
        tanks: vec![],
    };

    while let Some(row) = rows.try_next().await.unwrap() {
        user.user_id = row.get("userid");
        user.name = row.get("name");
        user.email = row.get("email");
        user.password = row.get("password");

    }
    user
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
