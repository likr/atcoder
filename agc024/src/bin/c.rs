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
    if a[0] > 0 || (1..n).any(|i| a[i - 1] + 1 < a[i]) {
        println!("-1");
        return;
    }
    let mut result = a[n - 1];
    let mut r = n - 1;
    for i in (1..n - 1).rev() {
        if a[i] + (r - i) > a[r] {
            result += a[i];
            r = i;
        }
    }
    println!("{}", result);
}
