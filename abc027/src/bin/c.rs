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
    let mut m = n / 2;
    let mut height = 0;
    while m > 0 {
        m /= 2;
        height += 1;
    }
    let mut x = 1;
    let mut count = 0;
    while x <= n {
        if height % 2 == count % 2 {
            x = x * 2 + 1;
        } else {
            x = x * 2;
        }
        count += 1;
    }
    if count % 2 == 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
