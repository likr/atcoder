use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn mod_pow(mut base: usize, mut exp: usize, modulus: usize) -> usize {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn binomial(n: usize, k: usize) -> usize {
    let mut a = 1;
    let mut b = 1;
    for i in 0..k {
        a = a * (n - i) % M;
        b = b * (k - i) % M;
    }
    a * inv(b) % M
}

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    if n == 2 {
        println!("0");
        return;
    }

    let mut result = mod_pow(2, n, M);
    result = result + M - 1;
    result = (result + M - binomial(n, a)) % M;
    result = (result + M - binomial(n, b)) % M;
    println!("{}", result);
}
