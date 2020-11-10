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
        k: i64,
        q: usize,
        a: [Usize1; q],
    }
    let mut score = vec![k - q as i64; n];
    for i in 0..q {
        score[a[i]] += 1;
    }
    debug!(score);
    for i in 0..n {
        if score[i] <= 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
