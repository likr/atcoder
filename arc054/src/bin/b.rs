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

fn t(x: f64, p: f64) -> f64 {
    x + p / 2f64.powf(x / 1.5)
}

fn dt(x: f64, p: f64) -> f64 {
    1. - 0.462098 * p * (-0.462098 * x).exp()
}

fn main() {
    input! {
        p: f64,
    }
    let mut l = 0.;
    let mut r = p;
    while r - l > 1e-10 {
        let m = (l + r) / 2.;
        if dt(m, p) < 0. {
            l = m;
        } else {
            r = m;
        }
    }
    let x = l;
    if x < 0. {
        println!("{}", t(0., p));
    } else {
        println!("{}", t(x, p));
    }
}
