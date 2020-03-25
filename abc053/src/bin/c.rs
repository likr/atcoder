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
    if x % 11 == 0 {
        println!("{}", (x / 11) * 2);
    } else if x % 11 > 6 {
        println!("{}", ((x + 10) / 11) * 2);
    } else {
        println!("{}", (x / 11) * 2 + 1);
    }
}
