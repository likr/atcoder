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
        ab: [(usize, usize); n],
    }
    let mut a = ab.iter().map(|&(ai, _)| ai).collect::<Vec<_>>();
    a.sort();
    let mut b = ab.iter().map(|&(_, bi)| bi).collect::<Vec<_>>();
    b.sort();
    if n % 2 == 0 {
        let amid2 = a[(n - 1) / 2] + a[n / 2];
        let bmid2 = b[(n - 1) / 2] + b[n / 2];
        println!("{}", bmid2 - amid2 + 1);
    } else {
        println!("{}", b[n / 2] - a[n / 2] + 1);
    }
}
