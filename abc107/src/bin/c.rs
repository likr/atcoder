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
        k: usize,
        x: [isize; n],
    }
    let mut result = INF as isize;
    for i in 0..=n - k {
        let d = if x[i] >= 0 && x[i + k - 1] >= 0 {
            x[i + k - 1]
        } else if x[i] < 0 && x[i + k - 1] < 0 {
            x[i].abs()
        } else {
            let a = x[i].abs();
            let b = x[i + k - 1];
            if a > b {
                a + 2 * b
            } else {
                b + 2 * a
            }
        };
        eprintln!("{}", d);
        result = min(result, d);
    }
    println!("{}", result);
}
