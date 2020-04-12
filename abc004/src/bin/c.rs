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
    let mut nums = (1..=6).collect::<Vec<usize>>();
    for i in 0..(n % 30) {
        nums.swap(i % 5, (i % 5) + 1);
    }
    for i in 0..6 {
        print!("{}", nums[i]);
    }
    println!("");
}
