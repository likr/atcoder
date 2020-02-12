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
        mut s: usize,
    }
    let mut nums = HashSet::new();
    let mut count = 1;
    while !nums.contains(&s) {
        nums.insert(s);
        count += 1;
        if s % 2 == 0 {
            s /= 2;
        } else {
            s = s * 3 + 1;
        }
    }
    println!("{}", count);
}
