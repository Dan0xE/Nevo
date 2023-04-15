use std::fs;
use std::io;
use std::path::Path;

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
                Ok(_) => {
                    println!("File copied successfully.");
                }
                Err(e) => match e.kind() {
                    io::ErrorKind::AlreadyExists => {
                        println!("File already exists, overwriting...");
                        fs::remove_file(&destination_file)?;
                        fs::copy(entry_path, destination_file)?;
                        println!("File overwritten successfully.");
                    }
                    _ => {
                        eprintln!("An error occurred: {:?}", e);
                        return Err(e);
                    }
                },
            }
        }
    }
    Ok(())
}
