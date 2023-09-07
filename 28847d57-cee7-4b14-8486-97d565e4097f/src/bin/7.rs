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
        k: usize,
        c: [usize; n],
    }
    let mut count = HashMap::new();
    for i in 0..k - 1 {
        *count.entry(c[i]).or_insert(0) += 1;
    }
    let mut result = 0;
    for i in k - 1..n {
        *count.entry(c[i]).or_insert(0) += 1;
        result = max(result, count.len());
        *count.entry(c[i + 1 - k]).or_insert(0) -= 1;
        if count[&c[i + 1 - k]] == 0 {
            count.remove(&c[i + 1 - k]);
        }
    }
    println!("{}", result);
}
