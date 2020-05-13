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
        k: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();

    let mut g = a[0];
    for i in 1..n {
        g = gcd(g, a[i]);
    }

    if a[0] >= k && k % g == 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
