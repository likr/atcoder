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
    }
    let mut result = 0;
    let mut max_count = 0;
    for i in 1..=n {
        let mut count = 0;
        let mut x = i;
        while x % 2 == 0 {
            count += 1;
            x /= 2;
        }
        if count >= max_count {
            result = i;
            max_count = count;
        }
    }
    println!("{}", result);
}
