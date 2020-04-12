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
    }
    let left = k - 1;
    let right = n - k;
    let mut count = 0;
    for &i in &[0, 1, 2] {
        for &j in &[0, 1, 2] {
            for &k in &[0, 1, 2] {
                let mut v = vec![i, j, k];
                v.sort();
                if v[1] == 1 {
                    if v[0] == 0 && v[2] == 1 {
                        count += left;
                    } else if v[0] == 0 && v[2] == 2 {
                        count += left * right;
                    } else if v[0] == 1 && v[2] == 1 {
                        count += 1;
                    } else {
                        count += right;
                    }
                }
            }
        }
    }
    eprintln!("{} {} {}", count, left, right);
    println!("{}", count as f64 / (n as f64).powi(3));
}
