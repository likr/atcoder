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
        a: [isize; n],
    }
    let mut cost = vec![0; n];
    cost[1] = (a[1] - a[0]).abs();
    for i in 2..n {
        cost[i] = min(
            cost[i - 2] + (a[i - 2] - a[i]).abs(),
            cost[i - 1] + (a[i - 1] - a[i]).abs(),
        );
    }
    println!("{}", cost[n - 1]);
}
