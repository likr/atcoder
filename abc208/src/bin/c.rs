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
        a: [usize; n],
    }
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| a[i]);
    let mut order = vec![0; n];
    for i in 0..n {
        order[indices[i]] = i;
    }

    for i in 0..n {
        if order[i] < k % n {
            println!("{}", k / n + 1);
        } else {
            println!("{}", k / n);
        }
    }
}
