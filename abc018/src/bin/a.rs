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
        a: usize,
        b: usize,
        c: usize,
    }
    if a > b && a > c {
        println!("1");
    } else if a > b || a > c {
        println!("2");
    } else {
        println!("3");
    }
    if b > a && b > c {
        println!("1");
    } else if b > a || b > c {
        println!("2");
    } else {
        println!("3");
    }
    if c > a && c > b {
        println!("1");
    } else if c > a || c > b {
        println!("2");
    } else {
        println!("3");
    }
}
