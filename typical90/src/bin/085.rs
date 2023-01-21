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
        k: usize,
    }
    let mut factors = HashSet::new();
    for d in 1.. {
        if d * d > k {
            break;
        }
        if k % d == 0 {
            factors.insert(d);
            factors.insert(k / d);
        }
    }
    let mut factors = factors.into_iter().collect::<Vec<_>>();
    factors.sort();
    let mut result = (factors.len() + 1) / 2;
    let n = factors.len();
    for i in 1..n {
        for j in 1..=i {
            let x = factors[i] * factors[j];
            if x > k {
                break;
            }
            if k % x == 0 && k / x >= factors[i] {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
