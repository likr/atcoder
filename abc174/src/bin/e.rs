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
        k: usize,
        a: [f64; n],
    }
    let mut l = 0.0;
    let mut r = 100000000000.0 as f64;
    while r - l > 0.00001 {
        let m = (l + r) / 2.;
        let mut count = 0;
        for i in 0..n {
            count += (a[i] / m) as usize;
        }
        if count <= k {
            r = m;
        } else {
            l = m;
        }
    }
    eprintln!("{} {}", l, r);
    println!("{}", (l + 0.5).round());
}
