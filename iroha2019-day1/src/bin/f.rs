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
        k: usize,
    }
    let mut m = n;
    let mut primes = vec![];
    for i in 2.. {
        if m == 1 || i * i > n {
            break;
        }
        while m % i == 0 {
            primes.push(i);
            m /= i;
        }
    }
    if m > 1 {
        primes.push(m);
    }
    // eprintln!("{:?}", primes);
    if primes.len() < k {
        println!("-1");
        return;
    }
    for i in 0..k - 1 {
        println!("{}", primes[i]);
    }
    let mut p = 1;
    for i in k - 1..primes.len() {
        p *= primes[i];
    }
    println!("{}", p);
}
