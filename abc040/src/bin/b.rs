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
    for i in 1..=n {
        for j in 1..=n / i {
            let d = (i as isize - j as isize).abs() as usize;
            result = min(result, d + (n - i * j));
        }
    }
    println!("{}", result);
}
