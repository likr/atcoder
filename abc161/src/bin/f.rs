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
    for k in 2..=n {
        let mut m = n;
        while m >= k {
            if m % k == 0 {
                m /= k;
            } else {
                m -= k;
            }
        }
        if m == 1 {
            result += 1;
            eprintln!("{}", k);
        }
    }
    println!("{}", result);
}
