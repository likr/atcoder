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
        a: [usize; n],
    }
    let mut count = vec![0; 9];
    for &ai in &a {
        if ai >= 3200 {
            count[8] += 1;
        } else {
            count[ai / 400] += 1;
        }
    }
    let mut result_min = 0;
    for i in 0..8 {
        if count[i] > 0 {
            result_min += 1;
        }
    }
    if result_min == 0 {
        println!("{} {}", 1, count[8]);
    } else {
        println!("{} {}", result_min, result_min + count[8]);
    }
}
