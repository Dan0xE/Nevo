use std::{fs, path::Path};

pub(crate) fn create_symlimk(
    origin: &str,
    destiniation: &str,
    shortcut_name: &str,
) -> Result<(), std::io::Error> {
    let shortcut_path = format!("{}\\{}.lnk", destiniation, shortcut_name);
    let path_check = Path::new(&shortcut_path);
    if !path_check.exists() {
        fs::create_dir_all(destiniation)?;
        std::os::windows::fs::symlink_file(origin, shortcut_path)?;
    }
    Ok(())
}
