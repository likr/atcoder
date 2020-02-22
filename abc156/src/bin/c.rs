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
        x: [isize; n],
    }
    let result = (1..=100)
        .map(|p| {
            let mut s = 0;
            for &xi in &x {
                s += (xi - p) * (xi - p);
            }
            s
        })
        .min()
        .unwrap();
    println!("{}", result);
}
