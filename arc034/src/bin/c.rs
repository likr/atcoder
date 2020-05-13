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
        a: usize,
        b: usize,
    }
    let mut factors = HashMap::new();
    for x in b + 1..=a {
        let mut y = x;
        for d in 2.. {
            if d * d > x {
                break;
            }
            while y % d == 0 {
                *factors.entry(d).or_insert(0) += 1;
                y /= d;
            }
        }
        if y > 1 {
            *factors.entry(y).or_insert(0) += 1;
        }
    }
    let mut result = 1usize;
    for &c in factors.values() {
        result = result * (c + 1) % M;
    }
    println!("{}", result);
}
