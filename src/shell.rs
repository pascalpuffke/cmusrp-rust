use std::process::Command;

pub fn get_stdout(cmd: &str, arg: &str) -> Option<String> {
    let output = Command::new(cmd)
        .arg(arg)
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        return Some(String::from_utf8_lossy(&output.stdout).to_string());
    }

    None
}
