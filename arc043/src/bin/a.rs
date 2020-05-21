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
        a: f64,
        b: f64,
        s: [usize; n],
    }
    let s_min = *s.iter().min().unwrap() as f64;
    let s_max = *s.iter().max().unwrap() as f64;
    if s_min == s_max {
        println!("-1");
        return;
    }
    let s = s.iter().map(|&si| si as f64).collect::<Vec<_>>();
    let p = b / (s_max - s_min);
    let mut avg = 0.;
    for i in 0..n {
        avg += s[i] * p;
    }
    avg /= n as f64;
    let q = a - avg;
    println!("{} {}", p, q);
}
