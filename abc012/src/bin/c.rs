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
    let mut s = 0;
    for i in 1..10 {
        for j in 1..10 {
            s += i * j;
        }
    }
    let x = s - n;
    for i in 1..10 {
        for j in 1..10 {
            if i * j == x {
                println!("{} x {}", i, j);
            }
        }
    }
}
