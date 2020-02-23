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
        x: [usize; n],
    }
    let mut count = 0;
    for k in 0..3usize.pow(n as u32) {
        let mut s = 1;
        let mut k = k;
        for i in 0..n {
            s *= x[i] - 1 + (k % 3);
            k /= 3;
        }
        if s % 2 == 0 {
            count += 1;
        }
    }
    println!("{}", count);
}
