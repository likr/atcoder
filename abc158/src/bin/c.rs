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
    }
    for i in 1..10000 {
        let x = i as f64;
        if (x * 0.08) as usize == a && (x * 0.1) as usize == b {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
