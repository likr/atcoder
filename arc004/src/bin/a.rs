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
        xy: [(f64, f64); n],
    }
    let mut result = 0.;
    for i in 0..n {
        for j in 0..i {
            let dx = xy[i].0 - xy[j].0;
            let dy = xy[i].1 - xy[j].1;
            let d = (dx * dx + dy * dy).sqrt();
            if d > result {
                result = d;
            }
        }
    }
    println!("{}", result);
}
