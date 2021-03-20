use std::process::Command;

pub fn get_stdout(cmd: &str, arg: &str) -> String {
    let output = Command::new(cmd)
        .arg(arg)
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        panic!("Process did not execute successfully: {} {} (exit code {}), make sure cmus is running!", cmd, arg, output.status.code().unwrap())
    }
}
