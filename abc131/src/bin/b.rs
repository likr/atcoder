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
        l: isize,
    }
    let t = (0..n).map(|i| l + i as isize).collect::<Vec<_>>();
    let e = t.iter().min_by_key(|&ti| (ti.abs(), ti)).unwrap();
    println!("{}", t.iter().sum::<isize>() - e);
}
