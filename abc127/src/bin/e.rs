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
    let mut f = vec![1; n * m + 1];
    let mut g = vec![1; n * m + 1];
    for i in 1..=n * m {
        f[i] = i * f[i - 1] % M;
        g[i] = inv(f[i]);
    }

    let mut result = 0usize;
    for d in 1..m {
        let md = d * (m - d) % M;
        let n2 = n * n % M;
        let h = f[n * m - 2] * g[k - 2] % M * g[n * m - k] % M;
        // eprintln!("{} {} {} {}", md, n2, h, md * n2 * h);
        result = (result + md * n2 % M * h % M) % M;
    }
    for d in 1..n {
        let nd = d * (n - d) % M;
        let m2 = m * m % M;
        let h = f[n * m - 2] * g[k - 2] % M * g[n * m - k] % M;
        // eprintln!("{} {} {} {}", nd, m2, h, nd * m2 * h);
        result = (result + nd * m2 % M * h % M) % M;
    }
    println!("{}", result);
}
