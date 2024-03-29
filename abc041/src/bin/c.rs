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
    let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|&(_, v)| v);
    a.reverse();
    for &(i, _) in &a {
        println!("{}", i + 1);
    }
}
