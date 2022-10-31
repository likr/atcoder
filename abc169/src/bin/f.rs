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

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = HashMap::new();
    dp.insert((0, 0), 1);
    let mut tmp = HashMap::new();
    for i in 0..n {
        for &(k, t) in dp.keys() {
            tmp.insert((k, t), dp[&(k, t)]);
        }
        for &(k, t) in dp.keys() {
            if t + a[i] > s {
                continue;
            }
            if let Some(&x) = tmp.get(&(k + 1, t + a[i])) {
                tmp.insert((k + 1, t + a[i]), (x + dp[&(k, t)]) % M);
            } else {
                tmp.insert((k + 1, t + a[i]), dp[&(k, t)]);
            }
        }
        std::mem::swap(&mut dp, &mut tmp);
        tmp.clear();
    }
    let mut pow2 = vec![1; n + 1];
    for i in 0..n {
        pow2[i + 1] = (pow2[i] * 2) % M;
    }
    let mut result = 0usize;
    for &(k, t) in dp.keys() {
        if t == s {
            result = (result + dp[&(k, t)] * pow2[n - k] % M) % M;
        }
    }
    println!("{}", result);
}
