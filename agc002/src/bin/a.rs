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
        a: isize,
        b: isize,
    }
    if a <= 0 && 0 <= b {
        println!("Zero");
    } else if 0 < b {
        println!("Positive");
    } else {
        let m = b - a + 1;
        if m % 2 == 0 {
            println!("Positive");
        } else {
            println!("Negative");
        }
    }
}
