use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        mut a: [i64; n],
        q: usize,
        b: [i64; q],
    }
    a.sort();
    for i in 0..q {
        let k = a.upper_bound(&b[i]);
        if k == 0 {
            println!("{}", (b[i] - a[k]).abs());
        } else if k == n {
            println!("{}", (b[i] - a[k - 1]).abs());
        } else {
            println!("{}", min((b[i] - a[k]).abs(), (b[i] - a[k - 1]).abs()));
        }
    }
}
