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
        p: usize,
        q: usize,
        a: [usize; n],
    }
    let mut result = 0usize;
    for i5 in 4..n {
        for i4 in 3..i5 {
            for i3 in 2..i4 {
                for i2 in 1..i3 {
                    for i1 in 0..i2 {
                        if a[i1] * a[i2] % p * a[i3] % p * a[i4] % p * a[i5] % p == q {
                            result += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", result);
}
