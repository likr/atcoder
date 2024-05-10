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
        x: usize,
        y: usize,
        z: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut ok = vec![false; n];
    let mut indices = (0..n).collect::<Vec<_>>();
    indices.sort_by_key(|&i| Reverse(a[i]));
    for &i in indices.iter().take(x) {
        ok[i] = true;
    }
    indices.sort_by_key(|&i| (Reverse(b[i]), i));
    for &&i in indices
        .iter()
        .filter(|&&i| !ok[i])
        .take(y)
        .collect::<Vec<_>>()
        .iter()
    {
        ok[i] = true;
    }
    indices.sort_by_key(|&i| (Reverse(a[i] + b[i]), i));
    for &&i in indices
        .iter()
        .filter(|&&i| !ok[i])
        .take(z)
        .collect::<Vec<_>>()
        .iter()
    {
        ok[i] = true;
    }
    for i in 0..n {
        if ok[i] {
            println!("{}", i + 1);
        }
    }
}
