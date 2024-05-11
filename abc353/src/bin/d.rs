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
const M: usize = 998244353;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn base(mut n: usize) -> usize {
    let mut ans = 1;
    while n > 0 {
        ans *= 10;
        n /= 10;
    }
    ans
}

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = (acc[i] + a[i]) % M;
    }
    let mut ans = 0;
    for j in 1..n {
        let b = base(a[j]);
        ans = (ans + (acc[j] * b) % M) % M;
        ans = (ans + (a[j] * j) % M) % M;
    }
    println!("{}", ans);
}
