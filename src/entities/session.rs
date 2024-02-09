use crate::api_keys::is_key_valid;
use crate::entities::user::User;
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
struct SessionRequest {
    user_id: u16,
    name: String,
    email: String,
}

// CREATE TABLE `session` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `sessionid` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`sessionid`)
// ) ENGINE=InnoDB;

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
