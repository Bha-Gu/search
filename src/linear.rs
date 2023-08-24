///Linear search
/// Runtime O(N)
pub fn linear<T>(arr: &[T], val: &T) -> Option<usize>
where
    T: PartialEq,
{
    // for i in 0..arr.len() {
    //     if arr[i] == val {
    //         return Some(i);
    //     }
    // }
    // None
    // Rust Linter Optimizer
    // (0..arr.len()).find(|&i| arr[i] == *val)
    arr.iter().position(|i| *i == *val)
}
