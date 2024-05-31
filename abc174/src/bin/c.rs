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
        k: usize,
    }
    let mut f = vec![false; k];
    let mut b = 0;
    for ans in 1.. {
        b = (b * 10 + 7) % k;
        if f[b % k] {
            println!("-1");
            return;
        }
        f[b % k] = true;
        if b % k == 0 {
            println!("{}", ans);
            return;
        }
    }
}
