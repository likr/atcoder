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

fn f(t: f64, a: f64, b: f64, c: f64) -> f64 {
    a * t + b * (c * t * PI).sin()
}

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    }
    let mut l = 0.;
    let mut r = 100.;
    loop {
        let m = (r + l) / 2.;
        let x = f(m, a, b, c);
        if (x - 100.).abs() <= 1e-7 {
            println!("{}", m);
            return;
        }
        if x < 100. {
            l = m;
        } else {
            r = m;
        }
    }
}
