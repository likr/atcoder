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
        ab: [(usize, usize); n],
    }
    let mut s = 0;
    for i in (0..n).rev() {
        let (ai, bi) = ab[i];
        if (ai + s) % bi != 0 {
            s += bi - (ai + s) % bi;
        }
    }
    println!("{}", s);
}
