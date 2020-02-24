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
        s: usize,
    }
    let mut l = (s as f64).sqrt() as usize;
    while l * l < s {
        l += 1;
    }
    let x3y2 = l * l - s;
    for x2 in 1..=1000000000 {
        if x3y2 % x2 == 0 {
            let y3 = x3y2 / x2;
            if y3 <= 1000000000 {
                println!("0 0 {} {} {} {}", x2, l, l, y3);
                break;
            }
        }
    }
}
