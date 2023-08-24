///Binary search
/// Runtime O(log(N))
pub fn binary<T>(arr: &Vec<T>, val: &T) -> Option<usize>
where
    T: PartialOrd + Clone,
{
    //lower pointer
    let mut lo: usize = 0;
    //higher pointer
    let mut hi: usize = arr.len();
    //middle pointer
    let mut m: usize = (lo + hi) / 2;
    //current item
    let mut curr: T = arr[m].to_owned();
    //looping till found or end
    while lo < hi {
        //found it
        if *val == curr {
            return Some(m);
        }
        //it could be in bottom segment
        else if curr > *val {
            //update top pointer
            hi = m;
        }
        //it could be in top segment
        else {
            //update bottom pointer
            lo = m + 1;
        }
        //update middle pointer
        m = (lo + hi) / 2;
        //update current item
        curr = arr[m].to_owned();
    }
    //not found
    None
}
