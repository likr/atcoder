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

fn main() {
    input! {
        n: usize,
    }
    let mut primes = vec![true; 55556];
    primes[0] = false;
    primes[1] = false;
    for p in 2..primes.len() {
        if !primes[p] {
            continue;
        }
        let mut f = 2;
        while p * f < primes.len() {
            primes[p * f] = false;
            f += 1;
        }
    }
    let primes = primes
        .into_iter()
        .enumerate()
        .filter(|(_, f)| *f)
        .map(|(p, _)| p)
        .filter(|p| p % 5 == 2)
        .collect::<Vec<_>>();
    for i in 0..n {
        print!("{} ", primes[i]);
    }
    println!("");
}
