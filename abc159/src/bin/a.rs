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
        m: usize,
    }
    let mut a = vec![0; n + m];
    for i in 0..m {
        a[i] = 1;
    }
    let mut count = 0;
    for i in 0..n + m {
        for j in 0..i {
            if i == j {
                continue;
            }
            if (a[i] + a[j]) % 2 == 0 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
