use sysinfo::{ProcessExt, System, SystemExt};

pub(crate) fn is_minecraft_running() -> bool {
    let processes = System::new_all();

    for process in processes.processes_by_exact_name("javaw.exe") {
        if process.name() == "javaw.exe" {
            return true;
        }
    }
    false
}
