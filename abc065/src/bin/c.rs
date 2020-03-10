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

fn factorial(n: usize) -> usize {
    let mut f = 1;
    for i in 2..=n {
        f = f * i % M;
    }
    f
}

fn main() {
    input! {
        n0: usize,
        m0: usize,
    }
    let n = max(n0, m0);
    let m = min(n0, m0);
    if n - m > 1 {
        println!("0");
    } else if n == m {
        println!("{}", 2 * factorial(n) * factorial(n) % M);
    } else {
        println!("{}", factorial(n) * factorial(m) % M);
    }
}
