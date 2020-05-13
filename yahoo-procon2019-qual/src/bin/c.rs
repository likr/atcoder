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
        k: usize,
        a: usize,
        b: usize,
    }
    if b <= a + 1 || k < a - 1 {
        println!("{}", k + 1);
        return;
    }
    let m = k + 1 - a;
    if m % 2 == 0 {
        println!("{}", a + (b - a) * (m / 2));
    } else {
        println!("{}", a + (b - a) * (m / 2) + 1);
    }
}
