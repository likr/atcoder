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
        w: [usize; n],
    }
    let mut bins = vec![INF; n];
    for i in 0..n {
        let wi = w[i];
        let mut k = i;
        for j in 0..n {
            if wi <= bins[j] && bins[j] < bins[k] {
                k = j;
            }
        }
        bins[k] = wi;
    }
    let mut count = 0;
    for i in 0..n {
        if bins[i] != INF {
            count += 1;
        }
    }
    println!("{}", count);
}
