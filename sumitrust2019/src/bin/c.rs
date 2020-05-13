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
        x: usize,
    }
    let n = (x + 104) / 105;
    let r = 105 * n - x;
    if 5 * n >= r {
        println!("1");
    } else {
        println!("0");
    }
}
