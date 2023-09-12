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
        l: i64,
        r: i64,
        a: [i64; n],
    }
    let mut f = vec![0; n + 1];
    for i in 0..n {
        f[i + 1] = min(f[i] + a[i], l * (i + 1) as i64);
    }
    let mut g = vec![0; n + 1];
    for i in (0..n).rev() {
        g[i] = min(g[i + 1] + a[i], r * (n - i) as i64);
    }
    let mut ans = INF as i64;
    for i in 0..=n {
        ans = min(ans, f[i] + g[i]);
    }
    println!("{}", ans);
}
