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
        a: [i64; n + 1],
        mut c: [i64; n + m + 1]
    }
    let mut b = vec![0; m + 1];
    for j in (0..=m).rev() {
        let mut s = 0;
        for i in 0..=m - j {
            debug!(i);
            if i <= n && j + i <= m {
                s += a[n - i] * b[j + i];
            }
        }
        debug!(c[n + j], s, a[n]);
        b[j] = (c[n + j] - s) / a[n];
    }
    println!(
        "{}",
        b.iter()
            .map(|bi| format!("{}", bi))
            .collect::<Vec<_>>()
            .join(" ")
    );
}