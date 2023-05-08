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
        a: [usize; n],
    }
    let mut count = vec![0; 200001];
    for i in 0..n {
        count[a[i]] += 1;
    }
    let mut result = 0usize;
    for i in 0..n {
        for x in 1.. {
            if x * x > a[i] {
                break;
            }
            if a[i] % x == 0 {
                let y = a[i] / x;
                if x == y {
                    result += count[x] * count[x];
                } else {
                    result += count[x] * count[y] * 2;
                }
            }
        }
    }
    println!("{}", result);
}
