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
    for i in 1..=n {
        f = (f * i) % M;
    }
    f
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
        x: [usize; n],
    }
    let mut d = vec![0; n - 1];
    for i in 1..n {
        d[i - 1] = x[i] - x[i - 1];
    }

    let mut g = vec![1; n];
    for i in 2..n {
        g[i] = (g[i - 1] + inv(i)) % M;
    }

    let f = factorial(n - 1);
    let mut result = 0usize;
    for k in 1..n {
        result = (result + d[k - 1] * g[k] % M) % M;
    }
    result = result * f % M;
    println!("{}", result);
}
