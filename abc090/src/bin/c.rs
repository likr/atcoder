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
        m: usize,
    }
    let count = if n > 1 && m > 1 {
        (n - 2) * (m - 2)
    } else if n > 1 {
        n - 2
    } else if m > 1 {
        m - 2
    } else {
        1
    };
    println!("{}", count);
}
