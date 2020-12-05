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

pub fn prime_factors(n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut m = n;
    for d in 2.. {
        if d * d > n {
            break;
        }
        let mut count = 0;
        while m % d == 0 {
            count += 1;
            m /= d;
        }
        if count > 0 {
            result.push((d, count));
        }
    }
    if m > 1 {
        result.push((m, 1));
    }
    result
}

fn main() {
    input! {
        n: usize,
    }
    let mut factors = vec![0; 31];
    for i in 2..=n {
        for (x, c) in prime_factors(i) {
            factors[x] = max(c, factors[x]);
        }
    }
    let mut s = 1;
    for i in 2..=n {
        for _ in 0..factors[i] {
            s *= i;
        }
    }
    s += 1;
    assert!(s <= 10000000000000);
    for i in 2..=n {
        assert!(s % i == 1);
    }
    println!("{}", s);
}
