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
        xc: [(usize, char); n],
    }
    let mut xc = xc
        .into_iter()
        .map(|(xi, ci)| (if ci == 'R' { 0 } else { 1 }, xi))
        .collect::<Vec<_>>();
    xc.sort();
    for &(_, xi) in &xc {
        println!("{}", xi);
    }
}
