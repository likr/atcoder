use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lrx: [(Usize1, Usize1, usize); q],
    }
    let mut count = vec![0; n + 1];
    let mut events = BinaryHeap::new();
    for i in 0..q {
        let (li, ri, _) = lrx[i];
        events.push(Reverse((li, 0, i)));
        events.push(Reverse((ri, 1, i)));
    }
    let mut start = vec![0; q];
    let mut stop = vec![0; q];
    for t in 0..n {
        while let Some(&Reverse((s, flag, i))) = events.peek() {
            if s == t && flag == 0 {
                start[i] = count[lrx[i].2];
                events.pop();
            } else {
                break;
            }
        }
        count[a[t]] += 1;
        while let Some(&Reverse((s, flag, i))) = events.peek() {
            if s == t && flag == 1 {
                stop[i] = count[lrx[i].2];
                events.pop();
            } else {
                break;
            }
        }
    }
    for i in 0..q {
        println!("{}", stop[i] - start[i]);
    }
}
