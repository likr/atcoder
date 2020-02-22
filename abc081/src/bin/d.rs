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
        a: [i64; n],
    }
    let min_index = (0..n).min_by_key(|&k| a[k]).unwrap();
    let max_index = (0..n).max_by_key(|&k| a[k]).unwrap();
    println!("{}", 2 * n - 1);
    if a[min_index].abs() > a[max_index].abs() {
        for i in 0..n {
            println!("{} {}", min_index + 1, i + 1);
        }
        for i in (1..n).rev() {
            println!("{} {}", i + 1, i);
        }
    } else {
        for i in 0..n {
            println!("{} {}", max_index + 1, i + 1);
        }
        for i in 1..n {
            println!("{} {}", i, i + 1);
        }
    }
}
