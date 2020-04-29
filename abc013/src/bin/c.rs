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
        n: i64,
        h: i64,
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
    }
    let mut result = std::i64::MAX;
    for z in 0..=n {
        let s = (e + d) * z - h - d * n;
        let t = b - d;
        let x = if s < 0 { 0 } else { max(0, s / t + 1) };
        let y = n - z - x;
        if y >= 0 {
            eprintln!("{} {} {}", x, y, z);
            result = min(result, a * x + c * y);
        }
    }
    println!("{}", result);
}
