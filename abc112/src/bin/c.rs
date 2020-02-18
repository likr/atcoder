use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        xyh: [(isize, isize, isize); n],
    }
    for cy in 0..=100 {
        for cx in 0..=100 {
            let mut h_set = HashSet::new();
            for &(xi, yi, hi) in &xyh {
                let dx = (cx - xi).abs();
                let dy = (cy - yi).abs();
                h_set.insert(hi + dx + dy);
            }
            for &h in &h_set {
                if xyh
                    .iter()
                    .all(|(xi, yi, hi)| *hi == max(0, h - (cx - xi).abs() - (cy - yi).abs()))
                {
                    println!("{} {} {}", cx, cy, h);
                    return;
                }
            }
        }
    }
}
