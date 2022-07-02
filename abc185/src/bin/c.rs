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
        l: usize,
    }
    let mut primes = vec![true; l + 1];
    for i in 2..=l {
        if primes[i] {
            for j in (2 * i..=l).step_by(i) {
                primes[j] = false;
            }
        }
    }
    let mut a = vec![0; l + 1];
    let mut b = vec![0; l + 1];
    for i in 0..11 {
        let mut x = l - i - 1;
        for j in 2..=l {
            while x % j == 0 {
                a[j] += 1;
                x /= j;
            }
        }
        let mut y = i + 1;
        for j in 2..=l {
            while y % j == 0 {
                b[j] += 1;
                y /= j;
            }
        }
    }
    let mut result = 1;
    for i in 0..=l {
        if primes[i] {
            result *= i.pow(a[i] - b[i]);
        }
    }
    println!("{}", result);
}
