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
        mut xc: [(usize, char); n],
    }
    xc.sort();
    for i in 0..n {
        if xc[i].1 == 'R' {
            println!("{}", xc[i].0);
        }
    }
    for i in 0..n {
        if xc[i].1 == 'B' {
            println!("{}", xc[i].0);
        }
    }
}
