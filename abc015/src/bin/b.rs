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
        a: [usize; n],
    }
    let mut s = 0;
    let mut c = 0;
    for &ai in &a {
        if ai != 0 {
            s += ai;
            c += 1;
        }
    }
    println!("{}", (s + c - 1) / c);
}
