use std::fs;
use std::io::BufRead;

pub fn load_keys_from_file() -> Vec<String> {
    let mut keys: Vec<String> = vec![];
    let file = fs::read("config/api_keys").unwrap();
    for line in file.lines() {
        keys.push(line.unwrap())
    }
    println!("{:#?}", keys);
    keys
}
