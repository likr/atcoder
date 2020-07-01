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

fn prime_factors(n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut m = n;
    for d in 2.. {
        if d * d > n {
            break;
        }
        let mut count = 0;
        while m % d == 0 {
            count += 1;
            m /= d;
        }
        if count > 0 {
            result.push((d, count));
        }
    }
    if m > 1 {
        result.push((m, 1));
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
        n: isize,
        m: usize,
    }
    let minus = n < 0;
    let n = n.abs() as usize;
    let factors = prime_factors(n);
    let k = dbg!(factors.iter().map(|&(_, c)| c).max().unwrap_or(0));

    let mut f = vec![1; m + k + 1];
    let mut g = vec![1; f.len()];
    for i in 2..f.len() {
        f[i] = f[i - 1] * i % M;
        g[i] = inv(f[i]);
    }

    let mut x = 1;
    for &(_, c) in &factors {
        x = x * f[c + m - 1] % M * g[c] % M * g[m - 1] % M;
    }
    dbg!(f[m + k - 1] * g[m - 1] % M * g[k] % M);
    let mut y = 0;
    for i in 0..=m {
        if (i % 2 == 1) == minus {
            y = (y + f[m] * g[i] % M * g[m - i] % M) % M;
        }
    }
    println!("{}", x * y % M);
}
