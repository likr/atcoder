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
    let mut l = 0;
    let mut s = 0usize;
    while l < n {
        let mut r = l + 1;
        while r < n && a[r - 1] < a[r] {
            r += 1;
        }
        let m = r - l;
        s += m * (m + 1) / 2;
        l = r;
    }
    println!("{}", s);
}
