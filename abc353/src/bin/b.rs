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
    let mut ans = 0usize;
    let mut sheet = 0;
    for i in 0..n {
        if sheet + a[i] > k {
            ans += 1;
            sheet = a[i];
        } else {
            sheet += a[i];
        }
    }
    if sheet > 0 {
        ans += 1;
    }
    println!("{}", ans);
}
