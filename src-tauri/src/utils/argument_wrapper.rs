use std::process::Command;
use std::{env, io};

pub(crate) fn argument_wrapper() -> io::Result<()> {
    let current_dir = env::current_dir().unwrap();

    let output = Command::new("powershell")
        .args(&[
            "-Executionpolicy", "bypass",
            "-Command", &format!("if (!([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {{ Start-Process PowerShell -Verb RunAs \"-NoProfile -ExecutionPolicy Bypass -Command 'cd $pwd; & '{}' '\" }}; $process = \"javaw.exe\"; Get-WmiObject Win32_Process -Filter \"name = '$process'\" | Select-Object CommandLine | % {{ $_ | Out-File -Width 10000 -FilePath '{}\\args.txt' -Encoding ascii -Append }}", env!("CARGO_MANIFEST_DIR"), current_dir.display())
        ])
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        println!("Successfully executed PowerShell command");
    } else {
        println!("Failed to execute PowerShell command");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to execute PowerShell command",
        ));
    }

    Ok(())
}
