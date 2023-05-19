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
        h1: usize,
        w1: usize,
        n: usize,
        h2: usize,
        w2: usize,
        a: [[usize; w1]; h1],
    }
    for k in 0..=h1 - h2 {
        let mut count = vec![0; n + 1];
        for i in 0..h1 {
            for j in 0..w1 {
                count[a[i][j]] += 1;
            }
        }
        for i in 0..h2 {
            for j in 1..w2 {
                count[a[k + i][j - 1]] -= 1;
            }
        }
        let mut result = vec![];
        for l in 0..=w1 - w2 {
            for i in 0..h2 {
                count[a[k + i][l + w2 - 1]] -= 1;
            }
            let mut c = 0usize;
            for i in 0..=n {
                if count[i] > 0 {
                    c += 1;
                }
            }
            result.push(format!("{}", c));
            for i in 0..h2 {
                count[a[k + i][l]] += 1;
            }
        }
        println!("{}", result.join(" "));
    }
}
