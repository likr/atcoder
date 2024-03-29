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
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut count = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if x == 500 * i + 100 * j + 50 * k {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
