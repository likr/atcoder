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

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        p: usize,
        q: usize,
        r: usize,
    }
    let a = vec![
        (p, q, r),
        (p, r, q),
        (q, p, r),
        (q, r, p),
        (r, p, q),
        (r, q, p),
    ];

    let mut result = 0;
    for &(x, y, z) in &a {
        result = max(result, (n / x) * (m / y) * (l / z));
    }

    println!("{}", result);
}
