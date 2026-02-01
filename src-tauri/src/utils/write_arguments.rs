use std::io;

pub(crate) fn write_args(arguments: String) -> io::Result<()> {
    let args_file = std::env::current_dir().unwrap().join("args.bat");
    if !args_file.exists() {
        std::fs::File::create(&args_file).unwrap();
    }

    pub(crate) fn read_args(&self) -> NevoResult<Vec<String>> {
        if self.file_path.exists() {
            return Err(
                std::io::Error::new(std::io::ErrorKind::NotFound, "File does not exist").into(),
            );
        };

        let mut args = Vec::new();
        let mut file = fs::File::open("argds.txt")?;

    Ok(())
}
