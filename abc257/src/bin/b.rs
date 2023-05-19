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
        q: usize,
        mut a: [usize; k],
        l: [Usize1; q],
    }
    for i in 0..q {
        if a[l[i]] < n && (l[i] + 1 == k || a[l[i]] + 1 != a[l[i] + 1]) {
            a[l[i]] += 1;
        }
    }
    let mut result = vec![];
    for i in 0..k {
        result.push(format!("{}", a[i]));
    }
    println!("{}", result.join(" "));
}
