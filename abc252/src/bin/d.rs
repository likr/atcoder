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
        a: [usize; n],
    }
    let mut count = HashMap::new();
    for i in 0..n {
        *count.entry(a[i]).or_insert(0) += 1;
    }
    let mut ans = n * (n - 1) * (n - 2) / 6;
    for &ai in count.keys() {
        if count[&ai] >= 2 {
            ans -= (n - count[&ai]) * count[&ai] * (count[&ai] - 1) / 2;
        }
        if count[&ai] >= 3 {
            ans -= count[&ai] * (count[&ai] - 1) * (count[&ai] - 2) / 6;
        }
    }
    println!("{}", ans);
}
