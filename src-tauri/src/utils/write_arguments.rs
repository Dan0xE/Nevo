use std::io;

pub(crate) fn write_args(arguments: String) -> io::Result<()> {
    let args_file = std::env::current_dir().unwrap().join("args.bat");
    if !args_file.exists() {
        std::fs::File::create(&args_file).unwrap();
    }

    std::fs::write(&args_file, arguments).unwrap();

    Ok(())
}
