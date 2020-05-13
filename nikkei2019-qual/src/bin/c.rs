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
        mut ab: [(isize, isize); n],
    }
    ab.sort_by_key(|&(ai, bi)| ai + bi);
    ab.reverse();
    let mut s_a = 0;
    let mut s_b = 0;
    for i in 0..n {
        if i % 2 == 0 {
            s_a += ab[i].0;
        } else {
            s_b += ab[i].1;
        }
    }
    println!("{}", s_a - s_b);
}
