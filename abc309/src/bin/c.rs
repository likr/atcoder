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
        k: usize,
        ab: [(usize, usize); n],
    }
    let mut count = HashMap::new();
    for &(ai, bi) in ab.iter() {
        *count.entry(ai).or_insert(0) += bi;
    }
    let mut ab = vec![(0, INF), (INF, 0)];
    for (&k, &v) in count.iter() {
        ab.push((k, v));
    }
    ab.sort();
    let m = ab.len();
    for i in (2..m).rev() {
        ab[i - 1].1 += ab[i].1;
    }
    debug!(ab);
    for i in 1..m {
        if ab[i].1 <= k {
            println!("{}", ab[i - 1].0 + 1);
            return;
        }
    }
}
