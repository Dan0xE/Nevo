use std::{
    fs,
    io::Read,
    path::{self},
};

pub(crate) fn read_args() -> Vec<String> {
    if path::Path::new("args.txt").exists() {
        println!("args.txt file exists");

        let mut args = Vec::new();
        let mut file = fs::File::open("args.txt").unwrap_or_else(|e| {
            println!("Error opening args.txt file: {}", e);
            std::process::exit(1);
        });

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap_or_else(|e| {
            println!("Error reading args.txt file: {}", e);
            std::process::exit(69);
        });

        for arg in contents.split_whitespace() {
            args.push(arg.to_string())
        }
        args
    } else {
        let mut result = Vec::new();
        result.push("File not found".to_string());
        result
    }
}
