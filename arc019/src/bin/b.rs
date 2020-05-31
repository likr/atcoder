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
        a: Chars,
    }
    let n = a.len();
    let mut diff_count = 0;
    for i in 0..n / 2 {
        if a[i] != a[n - 1 - i] {
            diff_count += 1;
        }
    }
    if diff_count == 0 {
        println!("{}", 25 * (n / 2 * 2));
    } else if diff_count == 1 {
        println!("{}", 25 * (n - 2) + 48);
    } else {
        println!("{}", 25 * n);
    }
}
