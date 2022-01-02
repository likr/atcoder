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
    }
    let mut count = vec![0; n + 1];
    for i in 2..=n {
        if count[i] == 0 {
            for j in (i..).step_by(i) {
                if j > n {
                    break;
                }
                count[j] += 1;
            }
        }
    }
    let mut result = 0;
    for i in 2..=n {
        if count[i] >= k {
            result += 1;
        }
    }
    println!("{}", result);
}
