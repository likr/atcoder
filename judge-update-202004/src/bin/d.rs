use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn gcd(a: usize, b: usize) -> usize {
    if b > a {
        gcd(b, a)
    } else if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        s: [usize; q],
    }
    let mut g = a.clone();
    for i in 1..n {
        g[i] = gcd(g[i - 1], g[i]);
    }
    for i in 0..q {
        let x = gcd(s[i], g[n - 1]);
        if x > 1 {
            println!("{}", x);
        } else {
            println!("{}", g.lower_bound_by(|&gi| 1.cmp(&gcd(s[i], gi))) + 1);
        }
    }
}
