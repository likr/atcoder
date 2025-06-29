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
        d: usize,
        p: usize,
        mut f: [usize; n],
    }
    f.sort();
    f.reverse();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + f[i];
    }
    let s = acc[n];
    let mut ans = s;
    for j in 0..=((n + d + 1) / d) {
        ans = min(ans, s - acc[min(n, j * d)] + p * j);
    }
    println!("{}", ans);
}
