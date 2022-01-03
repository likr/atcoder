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
        q: usize,
        mut a: [usize; n],
        txy: [(usize, usize, usize); q],
    }
    let mut offset = 0;
    for i in 0..q {
        let (ti, xi, yi) = txy[i];
        if ti == 1 {
            a.swap((xi - 1 + n - offset % n) % n, (yi - 1 + n - offset % n) % n);
        } else if ti == 2 {
            offset += 1;
        } else {
            println!("{}", a[(xi - 1 + n - offset % n) % n]);
        }
    }
}
