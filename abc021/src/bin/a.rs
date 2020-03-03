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
    let mut nums = vec![];
    let mut b = 1;
    while n > 0 {
        if n % 2 == 1 {
            nums.push(b)
        }
        n /= 2;
        b *= 2;
    }
    println!("{}", nums.len());
    for &i in &nums {
        println!("{}", i);
    }
}
