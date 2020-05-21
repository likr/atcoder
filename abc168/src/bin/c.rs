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
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let t1 = (h * 60. + m) / 720. * PI * 2.;
    let t2 = m / 60. * PI * 2.;
    let x1 = a * t1.cos();
    let y1 = a * t1.sin();
    let x2 = b * t2.cos();
    let y2 = b * t2.sin();
    let dx = x1 - x2;
    let dy = y1 - y2;
    println!("{}", (dx * dx + dy * dy).sqrt());
}
