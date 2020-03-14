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
        h: usize,
        w: usize,
    }
    if h == 1 || w == 1 {
        println!("1");
    } else if h % 2 == 0 || w % 2 == 0 {
        println!("{}", h * w / 2);
    } else {
        println!("{}", (h / 2) * w + w / 2 + 1);
    }
}
