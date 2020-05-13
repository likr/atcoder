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
        a: usize,
        b: usize,
    }
    if b < a || (n == 1 && a != b) {
        println!("0");
        return;
    }
    let s_min = a * (n - 1) + b;
    let s_max = a + b * (n - 1);
    println!("{}", s_max + 1 - s_min);
}
