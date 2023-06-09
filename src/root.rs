///Square root search
/// Runtime: O(√N)
pub fn root<T>(arr: &[T], val: &T) -> Option<usize>
where
    T: PartialOrd,
{
    let length = arr.len();
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    let jump = (length as f64).sqrt().floor() as usize;
    //let mut i =jump;
    // i = loop {
    //     if i >= length || arr[i] >= val {
    //         break i;
    //     }
    //     i += jump;
    // };
    // i -= jump
    let i = (jump..length).step_by(jump).find(|&j| arr[j] >= *val)? - jump;
    // for j in i..length.min(i + jump + 1) {
    //     if arr[j] == val {
    //         return Some(j);
    //     }
    // }
    // None
    // Rust Linter Optimizer
    (i..length.min(i + jump + 1)).find(|&j| arr[j] == *val)
}
