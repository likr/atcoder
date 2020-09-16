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
    let n = 100;
    let h = 1500;
    let w = 1500;
    let mut result = vec![];
    'outer: for k in (1..=n).rev() {
        for i in 0..=h {
            for j in 0..=w {
                if 0 <= i - k && i + k <= h && 0 <= j - k && j + k <= w {
                    if result.iter().all(|&(y, x, r)| {
                        (y - i) * (y - i) + (x - j) * (x - j) >= (k + r) * (k + r)
                    }) {
                        result.push((i, j, k));
                        continue 'outer;
                    }
                }
            }
        }
    }
    for &(x, y, _) in result.iter().rev() {
        println!("{} {}", x, y);
    }
}
