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
        k: usize,
        s: usize,
    }
    let mut count = 0;
    for x in 0..=k {
        for y in 0..=k {
            if x + y > s {
                break;
            }
            if s - x - y > k {
                continue;
            }
            count += 1;
        }
    }
    println!("{}", count);
}
