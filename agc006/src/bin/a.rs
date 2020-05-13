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
        s: Chars,
        t: Chars,
    }
    let mut m = 0;
    for k in 1..=n {
        if (0..k).all(|i| s[n - k + i] == t[i]) {
            m = k;
        }
    }
    println!("{}", 2 * n - m);
}
