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
    if a % 2 == 0 || b % 2 == 0 || c % 2 == 0 {
        println!("0");
    } else {
        let s = vec![b * c, a * c, a * b];
        println!("{}", s.iter().min().unwrap());
    }
}
