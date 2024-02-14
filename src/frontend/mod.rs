use std::process::Command;
use std::thread;

pub fn start_front_end() {
    thread::spawn(|| {
        Command::new("bash")
            .arg("-e")
            .arg("aquariquality-frontend/start-frontend.sh")
    });
}
