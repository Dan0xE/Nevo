// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::{fs, io, path::Path, process::Command};
use sysinfo::{ProcessExt, System, SystemExt};

#[derive(thiserror::Error, Debug)]
pub enum NevoError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parse Float Error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Arguments Error: {0}")]
    ArgumentsError(#[from] ArgumentsError),
}

impl Serialize for NevoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            NevoError::IoError(e) => serializer.serialize_str(&format!("IO Error: {}", e)),
            NevoError::ParseFloatError(e) => {
                serializer.serialize_str(&format!("Parse Float Error: {}", e))
            }
            NevoError::ArgumentsError(e) => {
                serializer.serialize_str(&format!("Arguments Error: {}", e))
            }
        }
    }
}

#[derive(thiserror::Error, Debug, Serialize)]
pub enum ArgumentsError {
    #[error("Arguments have not been generated yet.")]
    NotGenerated,
    #[error("Failed to find --version in arguments.")]
    MissingVersion,
    #[error("Failed to find -Djava.library.path in arguments.")]
    MissingPath,
    #[error("Failed to generate arguments: {0}")]
    GenerationFailed(String),
}

pub type NevoResult<T> = Result<T, NevoError>;

#[cfg(test)]
mod tests {
    mod copy_snapshot_test;
}

/// Check if Minecraft is running by looking for the `javaw.exe` process.
///
/// # Errors
/// This function will return an error if:
/// - Minecraft is not running
pub(crate) fn is_minecraft_running() -> NevoResult<()> {
    let processes = System::new_all();

    for process in processes.processes_by_exact_name("javaw.exe") {
        if process.name() == "javaw.exe" {
            return Ok(());
        }
    }

    Err(NevoError::ArgumentsError(ArgumentsError::GenerationFailed(
        "Minecraft is not running.".to_string(),
    )))
}

/// Copy all files from source directory to destination directory.
///
/// # Errors
/// This function will return an error if:
/// - the source or destination directory does not exist
/// - any file operation fails
pub(crate) fn copy_snapshot(source: &str, destination: &str) -> io::Result<()> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    // Check if source is dir
    if !source_path.is_dir() || !destination_path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source or destination dir not found",
        ));
    }

    for entry in source_path.read_dir()? {
        let entry = entry?;
        let entry_path = entry.path();

        // Check if entry is file
        if entry.file_type()?.is_file() {
            let destination_file = destination_path.join(entry.file_name());

            // Copy the file to destination
            match fs::copy(&entry_path, &destination_file) {
                Ok(_) => {}
                Err(e) => match e.kind() {
                    io::ErrorKind::AlreadyExists => {
                        fs::remove_file(&destination_file)?;
                        fs::copy(entry_path, destination_file)?;
                    }
                    _ => {
                        return Err(e);
                    }
                },
            }
        }
    }
    Ok(())
}

/// Holds information about the arguments used to launch the game
pub struct ArgumentsWrapper {
    /// The username to use when launching the cloned snapshot
    pub username: String,
    /// The path to the cloned snapshot
    pub snapshot_path: String,
    /// The arguments used to launch the game
    pub arguments: Option<Vec<String>>,
    /// Whether the version is a new version (1.18.2+)
    pub new_version: bool,
}

impl ArgumentsWrapper {
    pub fn new(username: String, snapshot_path: String) -> Self {
        Self {
            username,
            snapshot_path,
            arguments: None,
            new_version: false,
        }
    }

    /// Generate the arguments used to launch the game by querying the running `javaw.exe` process.
    ///
    /// # Errors
    /// This function will return an error if:
    /// - the PowerShell command fails to execute
    pub(crate) fn generate_arguments(&mut self) -> NevoResult<Vec<String>> {
        // TODO don't rely on powershell, find a way to get the arguments in a different way
        let output = Command::new("powershell")
            .args([
                "-ExecutionPolicy", "bypass",
                "-Command",
                "if (!([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) { Start-Process PowerShell -Verb RunAs \"-NoProfile -ExecutionPolicy Bypass -Command 'Get-WmiObject Win32_Process -Filter \"\"name = ''javaw.exe''\"\" | Select-Object CommandLine | Out-String -Width 10000'\" -Wait }; Get-WmiObject Win32_Process -Filter \"name = 'javaw.exe'\" | Select-Object CommandLine | Out-String -Width 10000"
            ])
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();

            // OPTIMIZE
            let arguments: Vec<String> = stdout.split_whitespace().map(|s| s.to_string()).collect();
            self.arguments = Some(arguments.clone());
            Ok(arguments)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(NevoError::ArgumentsError(ArgumentsError::GenerationFailed(
                format!("Failed to execute PowerShell command: {}", stderr),
            )))
        }
    }

    /// Adjust the arguments that the copied snapshot will use to launch the game.
    /// Namely, this function will change:
    /// - the `-Djava.library.path` to point to the new snapshot path
    /// - the `--username` to the provided username
    ///
    /// Please note that this functions assumes that the arguments have already been generated by `generate_arguments`.
    ///
    /// The function will also implicitly *mutate* `self.arguments` to store the adjusted arguments.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - the arguments have not been generated yet
    /// - the `--version` flag is missing from the arguments
    /// - the version number cannot be parsed as a float
    ///
    ///
    /// # Returns
    ///
    /// Returns the adjusted arguments.
    pub(crate) fn adjust_arguments(&mut self) -> NevoResult<Vec<String>> {
        let arguments = self.arguments.take();

        let mut arguments =
            arguments.ok_or(NevoError::ArgumentsError(ArgumentsError::NotGenerated))?;

        let mut args: Vec<String> = arguments.drain(0..2).collect();

        let mut new_version = false;

        // NOTE version should *never* be missing, however:
        // Instead of returning an error, we could just *assume* that it's a new version,
        // and set the new_version flag to true.
        let version_index = args
            .iter()
            .position(|e| e == "--version")
            .ok_or(NevoError::ArgumentsError(ArgumentsError::MissingVersion))?;

        // In versions > 1.18.1, the -Djava.library.path argument is no longer used.
        if args[version_index + 1].replace(".", "").parse::<f64>()? > 1181.0 {
            new_version = true;
        }

        for i in 0..args.len() {
            if !new_version && args[i].contains("-Djava.library.path") {
                let old = args[i].clone();
                args[i] = format!("-Djava.library.path={}", self.snapshot_path);
                copy_snapshot(&old[20..], &self.snapshot_path)?;
            }

            if args[i].contains("--username") {
                args[i + 1] = self.username.clone();
            }
        }

        self.new_version = new_version;
        // TODO avoid clone since it's expensive
        self.arguments = Some(args.clone());
        Ok(args)
    }

    pub(crate) fn launch_with_arguments(&self) -> NevoResult<()> {
        let arguments = self
            .arguments
            .as_ref()
            .ok_or(NevoError::ArgumentsError(ArgumentsError::NotGenerated))?;

        Command::new(&arguments[0]).args(&arguments[1..]).spawn()?;

        Ok(())
    }
}

#[tauri::command]
pub(crate) fn start_snapshot(username: String, path: String) -> NevoResult<()> {
    is_minecraft_running()?;

    let mut argument_wrapper = ArgumentsWrapper::new(username, path);
    argument_wrapper.generate_arguments()?;
    argument_wrapper.adjust_arguments()?;

    let arguments = argument_wrapper
        .arguments
        .as_ref()
        .ok_or(NevoError::ArgumentsError(ArgumentsError::NotGenerated))?;

    if !argument_wrapper.new_version {
        let djvava_lib_path = arguments
            .iter()
            .position(|e| e.contains(&"-Djava.library.path".to_string()))
            .ok_or(NevoError::ArgumentsError(ArgumentsError::MissingPath))?;

        copy_snapshot(
            &arguments[djvava_lib_path][20..],
            &argument_wrapper.snapshot_path,
        )?;
    }
    argument_wrapper.launch_with_arguments()?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_snapshot])
        .run(tauri::generate_context!())
        .expect("error while running nevo");
}
