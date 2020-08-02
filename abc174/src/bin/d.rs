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
        c: Chars,
    }
    let mut red_count = 0;
    for i in 0..n {
        if c[i] == 'R' {
            red_count += 1;
        }
    }
    let mut left_red_count = 0;
    for i in 0..red_count {
        if c[i] == 'R' {
            left_red_count += 1;
        }
    }
    println!("{}", red_count - left_red_count);
}
