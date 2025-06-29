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
    let mut ans = 0;
    let mut i = 1;
    while i <= n {
        let x = n / i;
        let mut ok = i;
        let mut ng = n + 1;
        while ng - ok > 1 {
            let k = (ok + ng) / 2;
            if n / k == x {
                ok = k;
            } else {
                ng = k;
            }
        }
        ans += x * (ng - i);
        i = ng;
    }
    println!("{}", ans);
}
