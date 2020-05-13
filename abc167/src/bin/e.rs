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
const M: usize = 998244353;

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

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    let mut f = vec![1; n + 1];
    let mut g = vec![1; n + 1];
    for i in 1..=n {
        f[i] = (i * f[i - 1]) % M;
        g[i] = inv(f[i]);
    }

    let mut result = 0usize;
    for i in 0..=k {
        let l = n - i;
        let mut s = m;
        s = s * modpow(m - 1, l - 1) % M;
        let c = f[n - 1] * g[i] % M * g[n - 1 - i] % M;
        s = s * c % M;
        result = (result + s) % M;
    }
    println!("{}", result);
}
