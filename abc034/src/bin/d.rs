use ordered_float::OrderedFloat;
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
        k: usize,
        wp: [(f64, f64); n],
    }
    let mut l = 0.;
    let mut r = 1.;
    for _ in 0..1000 {
        let m = (l + r) / 2.;
        let mut s = wp
            .iter()
            .map(|&(wi, pi)| wi * (pi / 100. - m))
            .collect::<Vec<_>>();
        s.sort_by_key(|&si| Reverse(OrderedFloat::from(si)));
        if (0..k).map(|i| s[i]).sum::<f64>() < 0. {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", l * 100.);
}
