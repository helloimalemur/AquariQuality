use crate::api_keys::is_key_valid;
use crate::entities::tank::Tank;
use crate::AppState;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{Data, Payload};
use actix_web::{error, web, HttpRequest, HttpResponse};
use futures_util::StreamExt;
use rand::{random, Rng};
use sqlx::{MySql, Pool, Row};
use std::sync::{Mutex, MutexGuard};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub user_id: i16,
    pub name: String,
    pub email: String,
    pub password: String,
    pub tanks: Vec<Tank>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// CREATE TABLE `user` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `password` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`userid`)
// ) ENGINE=InnoDB;

// curl -XPOST -H'x-api-key: omganotherone' localhost:8080/user/create/ -d '{"name":"johnny","email":"johhny@mail.com","password":"password"}'
pub async fn create_user_route(
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
            if let Ok(obj) = serde_json::from_slice::<UserRequest>(&body) {
                let mut rand = rand::thread_rng();
                let new_user_id: u16 = rand.gen();
                let user_req = obj.clone();
                let new_user = User {
                    user_id: new_user_id as i16,
                    name: obj.name,
                    email: obj.email,
                    password: obj.password,
                    tanks: vec![],
                };

                // println!("{:#?}", new_user.clone());
                let user_exists = check_user_exist(user_req.email, data.clone()).await;
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

pub async fn check_user_exist(user_email: String, mut data: Data<Mutex<AppState>>) -> bool {
    let mut user_exists: bool = false;
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

    // todo()! hash password

    let mut query_result_string = String::new();
    if let Ok(query_result_2) = sqlx::query("SELECT email FROM user WHERE email LIKE (?)")
        .bind(user_email)
        .fetch_one(&*db_pool)
        .await
    {
        query_result_string = query_result_2.get("email");
        if !query_result_string.is_empty() {
            user_exists = true;
        }
    }
    // println!("user exists: {}", user_exists);
    user_exists
}


pub async fn check_user_exist_with_password_hash(user_email: String, user_password: String, mut data: Data<Mutex<AppState>>) -> bool {
    let mut user_exists_and_password: bool = false;
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

    // todo()! hash password
    let password_hash = create_password_hash(user_password.clone());

    if let Ok(query_result) = sqlx::query("SELECT email,password FROM user WHERE email LIKE (?) AND password LIKE (?)")
        .bind(user_email.clone())
        .bind(user_password.clone())
        .fetch_one(&*db_pool)
        .await
    {
        if user_email.eq_ignore_ascii_case(query_result.get("email")) && password_hash.eq_ignore_ascii_case(query_result.get("password")) {
            user_exists_and_password = true;
        }
    }
    user_exists_and_password
}

pub fn create_password_hash(password: String) -> String {
    // todo!()
    password
}

pub async fn create_user(user: User, data: Data<Mutex<AppState>>) {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();
    let is_closed = db_pool.is_closed();
    // println!("Database connected: {}", !is_closed);

    // todo()! hash password

    let password_hash = create_password_hash(user.password.clone());

    let query_result =
        sqlx::query("INSERT INTO user (userid, name, email, password) VALUES (?,?,?,?)")
            .bind(user.user_id)
            .bind(user.name)
            .bind(user.email)
            .bind(password_hash)
            .execute(&*db_pool)
            .await
            .unwrap();

    // println!("{:#?}", query_result);
}

// curl -XPOST -H'x-api-key: 12790066417744034365' localhost:8080/api/delete/user/ -d '{"name":"johnny","email":"johhny@mail.com"}'
pub async fn delete_user_route(
    // name: web::Path<String>,
    mut payload: Payload,
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
                if (body.len() + chunk.len()) > MAX_SIZE {
                    return "request too large".to_string();
                }
                body.extend_from_slice(&chunk);
            }

            if let Ok(user) = serde_json::from_slice::<UserRequest>(&body) {
                if check_user_exist(user.clone().email, data.clone()).await {
                    if delete_user(user, data).await {
                        "user deleted".to_string()
                    } else {
                        "error deleting user".to_string()
                    }
                } else {
                    "user does not exist".to_string()
                }
            } else {
                "error deleting user".to_string()
            }
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub async fn delete_user(user: UserRequest, data: Data<Mutex<AppState>>) -> bool {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

    if let Ok(query_result) = sqlx::query("DELETE FROM user WHERE email LIKE (?)")
        .bind(user.email)
        .execute(&*db_pool)
        .await
    {
        true
    } else {
        false
    }
}

pub async fn modify_user_route(
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

pub fn modify_user(user: User, db_pool: Pool<MySql>) {}
