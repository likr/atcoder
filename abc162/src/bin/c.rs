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
        k: usize,
    }
    let mut gcd_count = HashMap::new();
    for a in 1..=k {
        for b in 1..=k {
            let x = gcd(a, b);
            *gcd_count.entry(x).or_insert(0) += 1;
        }
    }
    let mut s = 0;
    for (&x, count) in gcd_count.iter() {
        for c in 1..=k {
            s += count * gcd(x, c);
        }
    }
    println!("{}", s);
}
