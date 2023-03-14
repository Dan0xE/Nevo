pub(crate) fn argument_wrapper() -> bool {
    windows::initialize_sta().unwrap();

    return true;
}
