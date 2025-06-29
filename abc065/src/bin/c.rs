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
    }
    let (n, m) = (max(n, m), min(n, m));
    if n - m > 1 {
        println!("0");
        return;
    }
    let mut fact_n = 1;
    for i in 2..=n {
        fact_n = (fact_n * i) % M;
    }
    let mut fact_m = 1;
    for i in 2..=m {
        fact_m = (fact_m * i) % M;
    }
    if n == m {
        println!("{}", 2 * fact_n * fact_m % M);
    } else {
        println!("{}", fact_n * fact_m % M);
    }
}
