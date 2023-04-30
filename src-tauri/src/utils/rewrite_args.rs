use super::shift_array::shift_array;
use super::{
    copy_snapshot::copy_snapshot, generate_args::generate_args, launch_game::launch_game,
    read_arguments::read_args, write_arguments::write_args,
};

use std::error::Error;

fn rewrite_args(username: String, path: String) -> Result<(), Box<dyn Error>> {
    generate_args()?;
    let res = read_args()?;
    let mut args: Vec<String> = shift_array(res, 2);

    let mut new_version = false;
    let version_index = args.iter().position(|e| e == "--version").unwrap();
    if args[version_index + 1]
        .replace(".", "")
        .parse::<f64>()
        .unwrap()
        < 1181.0
    {
        new_version = true;
    }

    for i in 0..args.len() {
        if !new_version && args[i].contains("-Djava.library.path") {
            let old = args[i].clone();
            args[i] = format!("-Djava.library.path={}", path);
            if !path.is_empty() {
                copy_snapshot(&old[20..], &path)?;
            }
        } else if args[i].contains("--username") {
            if !username.is_empty() {
                args[i + 1] = username.clone();
            } else {
                eprintln!("no username specified");
            }
        }
    }
    write_args(args.join(" "))?;
    launch_game()?;

    Ok(())
}
