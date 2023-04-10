pub(crate) fn write_args(arguments: String) {
    let args_path = std::env::current_dir().unwrap_or_else(|e| {
        println!("Error getting current directory: {}", e);
        std::process::exit(1);
    });
    let args_file = args_path.join("args.bat");
    let stf = args_file.clone();
    if !args_file.exists() {
        std::fs::File::create(stf).unwrap();
    }
    std::fs::write(&args_file, arguments).unwrap_or_else(|e| {
        println!("Error writing arguments to file: {}", e);
    });
}
