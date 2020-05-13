use itertools::Itertools;
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
        c: [usize; n],
    }
    let mut count = c
        .iter()
        .group_by(|&c| c)
        .into_iter()
        .map(|(_, items)| items.count())
        .collect::<Vec<_>>();
    if count.len() == 1 {
        println!("-1");
        return;
    }
    if c[0] == c[n - 1] {
        count[0] += count.pop().unwrap();
    }
    count.sort();
    let s = count.pop().unwrap();
    println!("{}", (s - 1) / 2 + 1);
}
