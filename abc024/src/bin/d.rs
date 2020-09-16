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

fn inv(a: usize, m: usize) -> usize {
    let m = m as i64;
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

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let i = (b * c % M + M - a * c % M) * inv(a * b % M + a * c % M + M - b * c % M, M) % M;
    let j = (b * c % M + M - a * b % M) * inv(a * c % M + a * b % M + M - b * c % M, M) % M;
    println!("{} {}", i, j);
}
