pub(crate) fn shift_array<T>(mut arr: Vec<T>, count: usize) -> Vec<T> {
    for _ in 0..count {
        arr.remove(0);
    }
    arr
}
