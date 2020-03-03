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
    for i in 0..=n / 4 {
        for j in 0..=n / 7 {
            if 4 * i + 7 * j == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
