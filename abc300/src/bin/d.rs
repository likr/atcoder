use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
    }
    let mut primes = vec![true; 1000001];
    primes[0] = false;
    primes[1] = false;
    for i in 2..primes.len() {
        if primes[i] {
            for j in (2 * i..primes.len()).step_by(i) {
                primes[j] = false;
            }
        }
    }
    let primes = primes
        .iter()
        .enumerate()
        .filter(|&(_, &f)| f)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut result = 0;
    for i in 2..primes.len() {
        let c = primes[i];
        for j in 0..i - 1 {
            let a = primes[j];
            if a * a * c * c > n {
                break;
            }
            result += primes[j + 1..i].upper_bound_by_key(&(n / (c * c)), |b| a * a * b);
        }
    }
    println!("{}", result);
}
