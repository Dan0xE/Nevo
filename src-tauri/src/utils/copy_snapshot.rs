use std::fs;
use std::io;
use std::path::Path;

fn copy_files(source: &str, destination: &str) -> io::Result<()> {
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

            if (destination_file).exists() && entry.file_name() == "glfw.dll" {
                println!("Seems like a snapshot already exist, exiting loop");
                break;
            }

            // Copy the file to destination
            fs::copy(entry_path, destination_file)?;
        }
    }

    Ok(())
}

pub(crate) fn copy_snapshot(origin: String, destination: String) -> bool {
    match copy_files(&origin, &destination) {
        Ok(_) => {
            println!("Copied files");
            true
        }
        Err(e) => {
            eprintln!("Could not copy files: {}", e);
            false
        }
    }
}
