use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        t: [[usize; n]; n],
    }
    let mut x = (1..n).collect::<Vec<_>>();
    let mut result = 0;
    loop {
        let mut d = t[0][x[0]] + t[x[n - 2]][0];
        for i in 1..n - 1 {
            d += t[x[i - 1]][x[i]];
        }
        if d == k {
            result += 1;
        }
        if !x.next_permutation() {
            break;
        }
    }
    println!("{}", result);
}
