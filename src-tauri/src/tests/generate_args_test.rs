use crate::{
    utils::generate_args::generate_args, utils::is_minecraft_running::is_minecraft_running,
};

#[test]
fn generate_args_failure() {
    let result = generate_args();
    assert!(result.is_err());
}

#[test]
fn generate_args_sucess() {
    loop {
        println!("Please start minecraft to continue this test");
        if is_minecraft_running() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(5))
    }

    let result = generate_args();
    assert!(result.is_ok());
}
