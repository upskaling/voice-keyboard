use std::process::Command;

pub fn paplay(path: &str) {
    Command::new("paplay")
        .arg("--client-name=clavier-vocale")
        .arg(path)
        .output()
        .expect("paplay failed");
}
