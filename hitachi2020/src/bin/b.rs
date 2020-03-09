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
        na: usize,
        nb: usize,
        m: usize,
        a: [usize; na],
        b: [usize; nb],
        xyc: [(Usize1, Usize1, usize); m],
    }
    let a_min = *a.iter().min().unwrap();
    let b_min = *b.iter().min().unwrap();
    let mut result = a_min + b_min;
    for i in 0..m {
        let (xi, yi, ci) = xyc[i];
        result = min(result, a[xi] + b[yi] - ci);
    }
    println!("{}", result);
}
