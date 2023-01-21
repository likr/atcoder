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
        a: usize,
        b: usize,
        mut s: Chars,
    }
    let mut result = INF;
    for i in 0..n {
        let mut count = 0;
        for j in 0..n / 2 {
            if s[j] != s[n - 1 - j] {
                count += 1;
            }
        }
        result = min(result, a * i + b * count);
        let c = s[0];
        for j in 1..n {
            s[j - 1] = s[j];
        }
        s[n - 1] = c;
    }
    println!("{}", result);
}
