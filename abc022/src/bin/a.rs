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
        s: isize,
        t: isize,
        mut a: [isize; n],
    }
    for i in 1..n {
        a[i] += a[i - 1];
    }
    println!("{}", a.iter().filter(|&&ai| s <= ai && ai <= t).count());
}
