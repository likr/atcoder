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
        d: usize,
        x: [[i64; d]; n],
    }
    let mut count = 0;
    for j in 0..n {
        for i in 0..j {
            let mut d2 = 0;
            for k in 0..d {
                d2 += (x[i][k] - x[j][k]) * (x[i][k] - x[j][k]);
            }
            for k in 0.. {
                if k * k > d2 {
                    break;
                }
                if k * k == d2 {
                    count += 1;
                    break;
                }
            }
        }
    }
    println!("{}", count);
}
