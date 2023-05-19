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
        q: usize,
        s: Chars,
        lr: [(Usize1, Usize1); q],
    }
    let mut count = vec![0; n];
    for i in 1..n {
        if s[i - 1] == 'A' && s[i] == 'C' {
            count[i] = 1;
        }
    }
    for i in 1..n {
        count[i] += count[i - 1];
    }
    for i in 0..q {
        let (li, ri) = lr[i];
        println!("{}", count[ri] - count[li]);
    }
}
