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
        m: usize,
        s: [i64; n - 1],
        x: [i64; m],
    }
    let mut acc_left = vec![0; n];
    for i in 1..n {
        acc[i] = s[i - 1] - acc[i - 1];
    }
    debug!(acc_left);
    let mut acc_right = vec![0; n];
    for i in (1..n).rev() {
        acc[i - 1] = s[i - 1] - acc[i];
    }
    debug!(acc_right);
    let mut left = HashMap::new();
    let mut right_odd = HashMap::new();
    let mut right_even = HashMap::new();
    for i in 1..n {
        if i % 2 == 0 {
            *right_even.entry(acc[i]).or_insert(0) += 1;
        } else {
            *right_odd.entry(acc[i]).or_insert(0) += 1;
        }
    }
    for i in 0..n {}
    println!("{}", n);
}
