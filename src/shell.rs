use std::process::Command;

pub fn get_stdout(cmd: &str, arg: &str) -> String {
    let output = Command::new(cmd)
        .arg(arg)
        .output()
        .unwrap_or_else(|e| {
            panic!("Failed to execute process: {}", e)
        });

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).to_string()
    } else {
        panic!("Process did not execute successfully: {} {} (exit code {}), make sure cmus is running!", cmd, arg, output.status.code().unwrap())
    }
}