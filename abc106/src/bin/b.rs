use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
    let mut count = 0;
    for x in 1..=n {
        if x % 2 == 0 {
            continue;
        }
        let mut div_count = 0;
        for d in 1..=x {
            if x % d == 0 {
                div_count += 1;
            }
        }
        if div_count == 8 {
            count += 1;
        }
    }
    println!("{}", count);
}
