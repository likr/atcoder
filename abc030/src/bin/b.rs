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

fn min(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

fn max(x: f64, y: f64) -> f64 {
    if x < y {
        y
    } else {
        x
    }
}

fn main() {
    input! {
        mut n: f64,
        m: f64,
    }
    if n >= 12. {
        n -= 12.;
    }
    let t1 = 360. * (60. * n + m) / 720.;
    let t2 = 360. * m / 60.;
    eprintln!("{} {}", t1, t2);
    println!(
        "{}",
        min(max(t1, t2) - min(t1, t2), 360. + min(t1, t2) - max(t1, t2))
    );
}
