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
        mut p: [Usize1; n],
    }
    let mut count = 0;
    for i in 1..n {
        if p[i - 1] == i - 1 {
            p.swap(i - 1, i);
            count += 1;
        }
    }
    if p[n - 1] == n - 1 {
        count += 1;
    }
    println!("{}", count);
}
