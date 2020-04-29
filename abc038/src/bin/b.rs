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
        h1: usize,
        w1: usize,
        h2: usize,
        w2: usize,
    }
    if h1 == h2 || h1 == w2 || w1 == h2 || w1 == w2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
