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
    }
    let mut m = 1 << n;
    input! {
        mut a: [usize; 1 << n],
    }
    for i in 1..n {
        let mut b = vec![];
        for j in (0..a.len()).step_by(2) {
            b.push(max(a[j], a[j + 1]));
        }
        a = b;
    }
    debug!(a);
    println!("{}", min(a[0], a[1]));
}
