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
        mut a: [usize; n],
    }
    a.dedup();
    let n = a.len();
    let mut i = 1;
    let mut count = 1;
    while i < n - 1 {
        if !((a[i - 1] <= a[i] && a[i] <= a[i + 1]) || (a[i - 1] >= a[i] && a[i] >= a[i + 1])) {
            count += 1;
            i += 1;
        }
        i += 1;
    }
    println!("{}", count);
}
