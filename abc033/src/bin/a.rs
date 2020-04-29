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
        mut n: usize,
    }
    let mut nums = HashSet::new();
    for _ in 0..4 {
        nums.insert(n % 10);
        n /= 10;
    }
    if nums.len() == 1 {
        println!("SAME");
    } else {
        println!("DIFFERENT");
    }
}
