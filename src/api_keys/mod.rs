use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, Write};
use std::sync::Mutex;
use actix_web::{HttpRequest, web};
use actix_web::web::Data;
use rand::Rng;
use crate::AppState;

pub async fn create_api_key(
    // name: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(req.headers().get("x-api-key").unwrap().to_str().unwrap().to_string(), data.lock().unwrap().api_key.lock().unwrap().to_vec()) {
            let mut rng = rand::thread_rng();
            let new_key: u64 = rng.gen(); // generates a new api-key
            add_api_key_to_file(new_key.to_string());
            reload_state(data, load_keys_from_file());
            new_key.to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

pub async fn delete_api_key(
    key: web::Path<String>,
    data: Data<Mutex<AppState>>,
    req: HttpRequest,
) -> String {
    // verify api_key
    if req.headers().get("x-api-key").is_some() {
        if is_key_valid(req.headers().get("x-api-key").unwrap().to_str().unwrap().to_string(), data.lock().unwrap().api_key.lock().unwrap().to_vec()) {
            remove_api_key_from_file(key.to_string());
            reload_state(data, load_keys_from_file());
            "ok".to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
    }
}

fn reload_state(data: Data<Mutex<AppState>>, keys: Vec<String>) {
    data.lock().unwrap().api_key.lock().unwrap().clear();
    let new_vec: Vec<String> = vec![];
    for i in keys {
        data.lock().unwrap().api_key.lock().unwrap().push(i)
    }
}

pub fn load_keys_from_file() -> Vec<String> {
    let mut keys: Vec<String> = vec![];
    let file = fs::read("config/api_keys").unwrap();
    for line in file.lines() {
        keys.push(line.unwrap())
    }
    // println!("{:#?}", keys);
    keys
}

fn add_api_key_to_file(new_key: String) {
    let new_key_formatted = format!("{}\n", new_key);
    let mut opt = OpenOptions::new()
        .write(true)
        .append(true)
        .open("config/api_keys")
        .unwrap();
    opt.write(new_key_formatted.as_bytes()).unwrap();
}

fn remove_api_key_from_file(del_key: String) {
    // load current keys
    let mut keys: Vec<String> = vec![];
    let file = fs::read("config/api_keys").unwrap();
    for line in file.lines() {
        keys.push(line.unwrap())
    }
    // remove key
    let mut rewrite_keys: Vec<String> = vec![];
    for (i,u) in keys.iter().enumerate() {
        if !(*u == del_key) {
            rewrite_keys.push(u.to_string())
        }
    }

    // println!("{:#?}", rewrite_keys);

    let _ = fs::remove_file("config/api_keys");

    // write keys back to file
    let mut new_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .append(false)
        .open("config/api_keys")
        .unwrap();
    new_file.write("".as_bytes()).unwrap();

    for u in rewrite_keys {
        let formatted = format!("{}\n", u);
        new_file.write(formatted.as_bytes()).unwrap();
    }
}

pub fn is_key_valid(key_to_test: String, keys: Vec<String>) -> bool {
    keys.contains(&key_to_test)
}
