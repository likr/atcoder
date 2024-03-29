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
        a: [Usize1; n],
    }
    let mut f = vec![true; n];
    for i in 0..n {
        if f[i] {
            f[a[i]] = false;
        }
    }
    let result = f
        .iter()
        .enumerate()
        .filter(|&(_, &f)| f)
        .map(|(i, _)| format!("{}", i + 1))
        .collect::<Vec<_>>();
    println!("{}", result.len());
    println!("{}", result.join(" "));
}
