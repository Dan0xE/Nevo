use std::env;
use std::process::Command;

pub(crate) fn argument_wrapper() -> bool {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let output = Command::new("powershell")
        .args(&[
            "-Executionpolicy", "bypass",
            "-Command", &format!("cd \"{}\"; Start-Process powershell -Verb RunAs -ArgumentList '-Executionpolicy', 'bypass', '-Command', 'cd \"{}\"; Set-ExecutionPolicy bypass -Scope Process -Force; .\\p.ps1'", current_dir.display(), current_dir.display())
        ])
        .output()
        .expect("Failed to execute PowerShell command");

    match output.status.success() {
        true => true,
        false => false,
    }
}
