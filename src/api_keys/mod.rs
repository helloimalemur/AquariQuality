use std::fs;
use std::io::BufRead;
use std::sync::Mutex;
use actix_web::HttpRequest;
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
        if is_key_valid(req.headers().get("x-api-key").unwrap().to_str().unwrap().to_string(), data.clone().lock().unwrap().api_key.lock().unwrap().to_vec()) {
            let mut rng = rand::thread_rng();
            let y: u64 = rng.gen(); // generates a float between 0 and 1
            y.to_string()
        } else {
            "invalid api key\n".to_string()
        }
    } else {
        "invalid api key\n".to_string()
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

pub fn is_key_valid(key_to_test: String, keys: Vec<String>) -> bool {
    keys.contains(&key_to_test)
}
