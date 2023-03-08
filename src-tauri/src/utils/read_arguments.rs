use std::{
    fs,
    io::Read,
    path::{self},
};

pub(crate) fn read_args() -> Vec<String> {
    if path::Path::new("args.txt").exists() {
        println!("file exists");

        let mut args = Vec::new();
        let mut file = fs::File::open("args.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
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
