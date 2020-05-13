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
        a: isize,
        b: isize,
        _c: isize,
        mut k: isize,
    }
    let mut s = 0isize;
    if k >= a {
        s += a;
        k -= a;
    } else {
        s += k;
        k = 0;
    }
    if k >= b {
        k -= b;
    } else {
        k = 0;
    }
    s -= k;
    println!("{}", s);
}
