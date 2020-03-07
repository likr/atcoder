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
        mut a: [isize; n],
    }
    a.push(0);
    a.reverse();
    a.push(0);
    a.reverse();
    let n = a.len();
    let mut s = 0;
    for i in 1..n {
        s += (a[i - 1] - a[i]).abs();
    }
    for i in 1..n - 1 {
        println!(
            "{}",
            s - (a[i - 1] - a[i]).abs() - (a[i] - a[i + 1]).abs() + (a[i - 1] - a[i + 1]).abs()
        );
    }
}
