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
    let mut result = INF;
    for a in 1..n {
        let b = n - a;
        let mut x = a;
        let mut y = b;
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        while y > 0 {
            s += y % 10;
            y /= 10;
        }
        result = min(result, s);
    }
    println!("{}", result);
}
