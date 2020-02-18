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
    }
    let mut m = n;
    let mut s = 0;
    while m > 0 {
        s += m % 10;
        m /= 10;
    }
    if n % s == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
