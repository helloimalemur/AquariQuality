use std::process::Command;
use std::thread;

pub async fn start_front_end() {
    thread::spawn(|| {
        // Command::new("pwd")
        //     .spawn()
        //     .unwrap();
        Command::new("bash")
            .arg("-e")
            .arg("aquariquality-frontend/start-frontend.sh")
            .spawn()
            .unwrap();
    });
}

// #[cfg(test)]
// mod tests {
//     use crate::frontend::start_front_end;
//
//     #[tokio::test]
//     fn front_end() {
//         start_front_end();
//     }
// }
