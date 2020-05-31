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
        ab: [(usize, usize); n],
    }
    let mut a = ab.iter().map(|&(ai, _)| ai).collect::<Vec<_>>();
    let mut b = ab.iter().map(|&(_, bi)| bi).collect::<Vec<_>>();
    a.sort();
    b.sort();

    if n % 2 == 0 {
        let a_m = a[n / 2 - 1] + a[n / 2];
        let b_m = b[n / 2 - 1] + b[n / 2];
        println!("{}", b_m + 1 - a_m);
    } else {
        let a_m = a[n / 2];
        let b_m = b[n / 2];
        println!("{}", b_m + 1 - a_m)
    }
}
