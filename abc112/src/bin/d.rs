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
        m: usize,
    }
    if m % n == 0 {
        println!("{}", m / n);
        return;
    }
    let mut result = 1;
    for a0 in 2..m {
        if (n - 1) * a0 > m {
            break;
        }
        let a1 = m - (n - 1) * a0;
        if a1 < a0 {
            break;
        }
        if a1 % a0 == 0 {
            result = a0;
        }
    }
    println!("{}", result);
}
