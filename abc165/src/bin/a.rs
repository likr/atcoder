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
        k: usize,
        a: usize,
        b: usize,
    }
    let mut i = 1;
    while k * i <= b {
        if k * i >= a {
            println!("OK");
            return;
        }
        i += 1;
    }
    println!("NG");
}
