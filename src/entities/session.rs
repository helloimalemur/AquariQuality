use crate::api_keys::is_key_valid;
use crate::entities::user::{check_user_exist, create_user, User, UserRequest};
use crate::AppState;
use actix_web::error::ErrorBadRequest;
use actix_web::web::{Data, Payload};
use actix_web::{error, web, HttpRequest, HttpResponse};
use futures_util::{StreamExt, TryStreamExt};
use rand::{random, Rng};
use sqlx::mysql::MySqlRow;
use sqlx::{Error, MySql, Pool, Row};
use std::net::ToSocketAddrs;
use std::sync::{Mutex, MutexGuard};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Session {
    user_id: u16,
    name: String,
    email: String,
    session_id: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionId {
    session_id: String,
}

impl SessionId {
    pub fn new(string: String) -> SessionId {
        SessionId { session_id: string }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct LogoutRequest {
    email: String,
    session_id: String,
}

// CREATE TABLE `session` (
// `userid` INT NOT NULL,
// `name` VARCHAR(255) NOT NULL,
// `email` VARCHAR(255) NOT NULL,
// `sessionid` VARCHAR(255) NOT NULL,
// PRIMARY KEY (`sessionid`)
// ) ENGINE=InnoDB;

// curl -XPOST -H'x-api-key: omganotherone' localhost:8080/login/ -d '{"email":"johhny@mail.com","password":"password"}'
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
                // println!("{:#?}", login_request.clone());
                // verify user exists
                let user_exists = check_user_exist(login_req.email.clone(), data.clone()).await;
                if user_exists {
                    // process login and return session_id
                    let session_id = create_session(login_request, data.clone()).await;
                    println!(
                        "LOGIN SUCCESSFUL: {} :: {}",
                        login_req.email.clone(),
                        session_id
                    );
                    session_id.to_string()
                } else if !user_exists {
                    println!("LOGIN FAILED: {}", login_req.email.clone());
                    "user does not exist\n".to_string()
                } else {
                    println!("LOGIN FAILED: {}", login_req.email.clone());
                    "error logging in\n".to_string()
                }
            } else {
                println!("LOGIN FAILED");
                "error logging in\n".to_string()
            }
        } else {
            println!("LOGIN FAILED - INVALID API KEY");
            "invalid api key\n".to_string()
        }
    } else {
        println!("LOGIN FAILED - INVALID API KEY");
        "invalid api key\n".to_string()
    }
}

async fn get_user_from_login_request(
    user_login_request: LoginRequest,
    db_pool: Pool<MySql>,
) -> Result<User, sqlx::Error> {
    // println!("{}", "attempting login");
    let mut user = sqlx::query("SELECT * FROM user WHERE email LIKE (?) AND password LIKE (?)")
        .bind(user_login_request.email)
        .bind(user_login_request.password)
        .map(|row: MySqlRow| User {
            user_id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            password: row.get(3),
            tanks: vec![],
        })
        .fetch_one(&db_pool)
        .await;

    user
}

pub async fn create_session(
    user_login_request: LoginRequest,
    data: Data<Mutex<AppState>>,
) -> String {
    let mut app_state = data.lock();
    let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();

    // query user from db using login request
    if let Ok(user) = get_user_from_login_request(user_login_request, db_pool.clone()).await {
        // create session token
        let new_session_id = generate_jwt_session_id(user.user_id).await;

        // delete any old sessions prior to creating new session
        delete_session_by_userid(user.user_id, db_pool.clone()).await;

        let userid = user.user_id.clone();
        let email = user.email.clone();

        if let Ok(query_result) =
            sqlx::query("INSERT INTO session (userid,name,email,sessionid) VALUES (?,?,?,?)")
                .bind(user.user_id)
                .bind(user.name)
                .bind(user.email)
                .bind(new_session_id.clone())
                .execute(&*db_pool)
                .await
        {
            println!(
                "SESSION CREATED :: {} :: {} :: {}",
                userid,
                email,
                new_session_id.clone()
            );
            new_session_id.to_string()
        } else {
            "null".to_string()
        }
    } else {
        return "null".to_string();
    }
}

async fn generate_jwt_session_id(user_id: i16) -> String {
    let mut rand = rand::thread_rng();
    let temp_new_session_id: u128 = rand.gen();
    temp_new_session_id.to_string()
}


// curl -XPOST -H'x-api-key: omganotherone' localhost:8080/logout/ -d '{"email":"johhny@mail.com","session_id":"password"}'
pub async fn logout_user_route(
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
            if let Ok(obj) = serde_json::from_slice::<LogoutRequest>(&body) {
                let logout_rq = obj.clone();
                let logout_request = LogoutRequest {
                    session_id: obj.session_id,
                    email: obj.email,
                };

                // println!("{:#?}", logout_request.clone());
                let user_exists = check_user_exist(logout_rq.email.clone(), data.clone()).await;
                let mut app_state = data.lock();
                let mut db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();
                let user_session_exists = check_if_session_exists(SessionId::new(logout_rq.session_id.clone()), db_pool.clone()).await;


                if user_exists && user_session_exists {
                    // process login
                    let mut app_state = data.lock();
                    let db_pool = app_state.as_mut().unwrap().db_pool.lock().unwrap();
                    delete_session_by_sessionid(logout_rq.session_id.clone(), db_pool.clone())
                        .await;

                    println!(
                        "LOGOUT SUCCESSFUL: {} :: {}",
                        logout_rq.email.clone(),
                        logout_rq.session_id.clone()
                    );
                    "user logout successful\n".to_string()
                } else if !user_exists {
                    println!(
                        "LOGOUT FAILED, USER DOES NOT EXIST: {} :: {}",
                        logout_rq.email.clone(),
                        logout_rq.session_id.clone()
                    );
                    "user does not exist\n".to_string()
                } else {
                    println!(
                        "LOGOUT FAILED: {} :: {}",
                        logout_rq.email.clone(),
                        logout_rq.session_id.clone()
                    );
                    "error logging out\n".to_string()
                }
            } else {
                println!("LOGOUT FAILED");
                "error logging out\n".to_string()
            }
        } else {
            println!("LOGOUT FAILED - INVALID API KEY");
            "invalid api key\n".to_string()
        }
    } else {
        println!("LOGOUT FAILED - INVALID API KEY");
        "invalid api key\n".to_string()
    }
}

pub async fn delete_session_by_userid(user_id: i16, db_pool: Pool<MySql>) {
    if let Ok(query_result) = sqlx::query("DELETE FROM session WHERE userid=(?)")
        .bind(user_id.clone())
        .execute(&db_pool)
        .await
    {
        println!("SESSION DELETED :: {}", user_id.clone());
    }
}

pub async fn delete_session_by_email(email: String, db_pool: Pool<MySql>) {
    if let Ok(query_result) = sqlx::query("DELETE FROM session WHERE email=(?)")
        .bind(email.clone())
        .execute(&db_pool)
        .await
    {
        println!("SESSION DELETED :: {}", email.clone());
    }
}

pub async fn delete_session_by_sessionid(session_id: String, db_pool: Pool<MySql>) {
    if let Ok(query_result) = sqlx::query("DELETE FROM session WHERE sessionid=(?)")
        .bind(session_id.clone())
        .execute(&db_pool)
        .await
    {
        println!("SESSION DELETED :: {}", session_id);
    }
}

pub async fn check_if_session_exists(session_id: SessionId, db_pool: Pool<MySql>) -> bool {
    println!("session check:");
    if session_id.session_id.len() > 0 {
        let result = sqlx::query("SELECT * FROM session WHERE sessionid=(?)")
            .bind(session_id.session_id.to_string())
            .fetch_all(&db_pool)
            .await
            .unwrap();

        return if let Some(row1) = result.get(0) {
            // println!("{:#?}", row1);
            // let b: String = row1.get(3);
            let stored_sessionid: String = row1.get("sessionid");
            println!("{}", stored_sessionid);

            if stored_sessionid.eq_ignore_ascii_case(session_id.session_id.as_str()) {
                true
            } else {
                false
            }
        } else {
            false
        }

    } else {
        false
    }
}

pub async fn check_if_session_exists_with_user_id(user_id: i16, session_id: SessionId, db_pool: Pool<MySql>) -> bool {
    println!("{} - {}", user_id, session_id.session_id);
    let result = sqlx::query("SELECT (1) FROM session WHERE sessionid=(?) AND userid=(?)")
        .bind(session_id.session_id)
        .bind(user_id)
        .fetch_all(&db_pool)
        .await
        .unwrap();


    // let queried_sessionid: String = result.get("sessionid").unwrap();
    // println!("{}", queried_sessionid);


    // let queried_user_id: i16 = result.get("userid").unwrap();


    true
}

#[cfg(test)]
mod tests {
    use crate::api_keys::load_keys_from_file;
    use crate::entities::session::{create_session, LoginRequest};
    use crate::AppState;
    use actix_web::web::Data;
    use config::Config;
    use sqlx::MySqlPool;
    use std::collections::HashMap;
    use std::sync::Mutex;

    #[actix_rt::test]
    pub async fn test_session_insert() {
        let data = setup_test().await;
        let login_request = LoginRequest {
            email: "email@mail.com".to_string(),
            password: "password".to_string(),
        };

        // println!("{:#?}", login_request.clone());

        let session_id = create_session(login_request, data.clone()).await;
    }

    async fn setup_test() -> Data<Mutex<AppState>> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config/Settings"))
            .build()
            .expect("could not load Settings.toml");
        let settings_map = settings
            .try_deserialize::<HashMap<String, String>>()
            .expect("unable to deserialize settings");

        let database_url = settings_map
            .get("database_url")
            .expect("could not get database_url from settings");

        // database connection
        let db_pool = MySqlPool::connect(database_url)
            .await
            .expect("unable to connect to database");

        let state = Data::new(Mutex::new(AppState::new(
            load_keys_from_file(),
            db_pool.clone(),
        )));
        state
    }
}
