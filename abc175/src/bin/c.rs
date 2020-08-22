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
        mut x: isize,
        k: isize,
        d: isize,
    }
    x = x.abs();
    let l = x / d;
    let result = if l > k {
        x - d * k
    } else {
        if (k - l) % 2 == 0 {
            x - d * l
        } else {
            ((x - d * l) - d).abs()
        }
    };
    println!("{}", result);
}
