pub(crate) fn write_args(arguments: String) {
    let args_path = std::env::current_dir().unwrap();
    let args_file = args_path.join("args.bat");
    let stf = args_file.clone();
    if !args_file.exists() {
        std::fs::File::create(stf).unwrap();
    }
    std::fs::write(&args_file, arguments).unwrap();
}
