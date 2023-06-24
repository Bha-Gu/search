mod linear;
use linear::linear;

mod binary;
use binary::binary;

mod root;
use root::root;

fn main() {
    let arr = vec![1, 2, 4, 23, 5, 6, 3, 7, 9, 8, 93];
    let mut sorted = arr.clone();
    sorted.sort_unstable(); // unstable sort is faster on primitive types
    println!(
        "L: {:?}\nB: {:?}\nR: {:?}",
        linear(&arr, &23),
        binary(&sorted, &23),
        root(&sorted, &23)
    );
}
