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
        na: usize,
        nb: usize,
        a: [usize; na],
        b: [usize; nb],
    }
    let a = a.iter().collect::<HashSet<_>>();
    let mut s = 0;
    for &bi in &b {
        if a.contains(&bi) {
            s += 1;
        }
    }
    println!("{}", s as f64 / (na + nb - s) as f64);
}
