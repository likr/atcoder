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
        _n: usize,
        m: usize,
        mut lr: [(usize, usize); m],
    }
    let l = lr.iter().map(|&(li, _)| li).max().unwrap();
    let r = lr.iter().map(|&(_, ri)| ri).min().unwrap();
    if l <= r {
        println!("{}", r - l + 1);
    } else {
        println!("0");
    }
}
