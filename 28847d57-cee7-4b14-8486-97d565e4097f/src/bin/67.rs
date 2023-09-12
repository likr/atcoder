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
        mut x: Chars,
    }
    let n = x.len();
    let mut x = x
        .into_iter()
        .map(|c| c as usize - '0' as usize)
        .collect::<Vec<_>>();
    let mut acc = x.clone();
    for i in 1..n {
        acc[i] += acc[i - 1];
    }
    x.reverse();
    let mut s = 0;
    for i in 0..n {
        x[i] = (s + acc[n - 1 - i]) % 10;
        s = (s + acc[n - 1 - i]) / 10;
    }
    while s > 0 {
        x.push(s % 10);
        s /= 10;
    }
    x.reverse();
    println!(
        "{}",
        x.into_iter().map(|c| format!("{}", c)).collect::<String>()
    );
}
