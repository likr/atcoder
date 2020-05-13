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

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        return gcd(b, a);
    }
    if a % b == 0 {
        return b;
    }
    gcd(b, a % b)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let k = gcd(n, m);
    let n0 = n / k;
    let m0 = m / k;
    if (0..k).all(|i| s[i * n0] == t[i * m0]) {
        println!("{}", n * m / k);
    } else {
        println!("-1");
    }
}
