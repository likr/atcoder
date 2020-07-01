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

fn modpow(x: usize, y: usize) -> usize {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % M;
        }
        a = a * a % M;
        b /= 2;
    }
    result
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = vec![0; n];
    for &ai in &a {
        count[ai] += 1;
    }
    let max_a = *a.iter().max().unwrap();
    if a[0] != 0 || count[0] != 1 || (0..=max_a).any(|i| count[i] == 0) {
        println!("0");
        return;
    }

    let mut x = 1;
    for i in 1..=max_a {
        x = x * modpow(2, count[i] * (count[i] - 1) / 2) % M;
        x = x * modpow(modpow(2, count[i - 1]) - 1, count[i]) % M;
    }
    println!("{}", x);
}
