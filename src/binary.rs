///Binary search
/// Runtime O(log(N))
pub fn binary<T>(arr: &Vec<T>, val: &T) -> Option<usize>
where
    T: PartialOrd + Clone,
{
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();
    let mut m: usize = (lo + hi) / 2;
    let mut curr: T = arr[m].to_owned();
    while lo < hi {
        if *val == curr {
            return Some(m);
        } else if curr > *val {
            hi = m;
        } else {
            lo = m + 1;
        }
        m = (lo + hi) / 2;
        curr = arr[m].to_owned();
    }
    None
}
