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
        l: usize,
        r: usize,
        d: usize,
    }
    let mut count = 0;
    for i in l..=r {
        if i % d == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
