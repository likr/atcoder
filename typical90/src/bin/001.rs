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
        l: i64,
        k: usize,
        a: [i64; n],
    }
    let mut ok = 0;
    let mut ng = l;
    while ng - ok > 1 {
        let m = (ng + ok) / 2;
        let mut count = 0;
        let mut start = 0;
        for i in 0..n {
            if a[i] - start >= m {
                start = a[i];
                count += 1;
            }
        }
        if l - start >= m {
            count += 1;
        }
        if count >= k + 1 {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{}", ok);
}
