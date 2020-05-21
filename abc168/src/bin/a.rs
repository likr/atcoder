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
    }
    let d = n % 10;
    if d == 2 || d == 4 || d == 5 || d == 7 || d == 9 {
        println!("hon");
    } else if d == 3 {
        println!("bon");
    } else {
        println!("pon");
    }
}
