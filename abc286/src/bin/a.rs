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
        p: Usize1,
        q: Usize1,
        r: Usize1,
        _s: Usize1,
        a: [usize; n],
    }
    let mut b = a.clone();
    for i in 0..=q - p {
        b[p + i] = a[r + i];
        b[r + i] = a[p + i];
    }
    println!(
        "{}",
        b.iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
