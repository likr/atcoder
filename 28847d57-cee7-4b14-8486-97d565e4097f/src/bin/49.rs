use ac_library::*;
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
    }
    let mut ans = k;
    if n > 1 {
        ans = (ans * (k - 1)) % M;
    }
    if n > 2 {
        if k > 1 {
            ans = (ans * pow_mod(k as i64 - 2, n as i64 - 2, M as u32) as usize) % M;
        } else {
            ans = 0;
        }
    }
    println!("{}", ans);
}
