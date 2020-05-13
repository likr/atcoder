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

fn min(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}

fn main() {
    input! {
        x: f64,
        y: f64,
        n: usize,
        mut xy: [(f64, f64); n],
    }
    xy.push(xy[0]);
    let mut result = INF as f64;
    for i in 0..n {
        let (x1, y1) = xy[i];
        let (x2, y2) = xy[i + 1];
        if x1 == x2 {
            result = min(result, (x1 - x).abs());
        } else if y1 == y2 {
            result = min(result, (y1 - y).abs());
        } else {
            let a = (y2 - y1) / (x2 - x1);
            let b = -1.;
            let c = y1 - a * x1;
            let d = (a * x + b * y + c).abs() / (a * a + b * b).sqrt();
            result = min(result, d);
        }
    }
    println!("{}", result);
}
