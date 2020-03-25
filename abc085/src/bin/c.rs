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
        y: usize,
    }
    for a in 0..=n {
        for b in 0..=n {
            if a + b > n {
                continue;
            }
            let c = n - a - b;
            if 10000 * a + 5000 * b + 1000 * c == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
