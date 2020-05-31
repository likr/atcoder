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

fn f(mut x: isize) -> isize {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn main() {
    input! {
        n: isize,
    }
    let mut result = vec![];
    for x in max(0, n - 153)..=n {
        if n == x + f(x) {
            result.push(x)
        }
    }
    println!("{}", result.len());
    for &z in &result {
        println!("{}", z);
    }
}
