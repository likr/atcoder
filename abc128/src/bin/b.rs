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
        mut sp: [(String, usize); n],
    }
    let mut sp = sp
        .into_iter()
        .enumerate()
        .map(|(i, (si, pi))| (i + 1, si, pi))
        .collect::<Vec<_>>();
    sp.sort_by_key(|row| (row.1.clone(), Reverse(row.2)));
    for i in 0..n {
        println!("{}", sp[i].0);
    }
}
