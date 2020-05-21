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

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    input! {
        x1: f64,
        y1: f64,
        r: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    }
    if x2 <= x1 - r && x1 + r <= x3 && y2 <= y1 - r && y1 + r <= y3 {
        println!("NO");
    } else {
        println!("YES");
    }
    if dist(x1, y1, x2, y2) <= r
        && dist(x1, y1, x2, y3) <= r
        && dist(x1, y1, x3, y2) <= r
        && dist(x1, y1, x3, y3) <= r
    {
        println!("NO");
    } else {
        println!("YES");
    }
}
