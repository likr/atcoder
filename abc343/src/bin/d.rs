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
        t: usize,
        ab: [(Usize1, usize); t],
    }
    let mut points = vec![0; n];
    let mut count = HashMap::new();
    count.insert(0, n);
    for &(ai, bi) in ab.iter() {
        *count.entry(points[ai]).or_insert(0) -= 1;
        if count[&points[ai]] == 0 {
            count.remove(&points[ai]).unwrap();
        }
        points[ai] += bi;
        *count.entry(points[ai]).or_insert(0) += 1;
        println!("{}", count.len())
    }
}
