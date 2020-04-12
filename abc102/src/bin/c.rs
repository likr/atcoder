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
    let mut a = a
        .into_iter()
        .enumerate()
        .map(|(i, ai)| ai - (i as isize + 1))
        .collect::<Vec<_>>();
    a.sort();

    let b = if n % 2 == 0 {
        (a[n / 2 - 1] + a[n / 2]) / 2
    } else {
        a[n / 2]
    };
    let mut result = 0;
    for i in 0..n {
        result += (a[i] - b).abs();
    }
    println!("{}", result);
}
