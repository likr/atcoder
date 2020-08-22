use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n],
    }
    let mut candidates = vec![];
    for i in 0..k {
        candidates.push((x[i], i));
    }
    candidates.sort();
    println!("{}", candidates[k - 1].1 + 1);
    for i in k..n {
        let item = (x[i], i);
        let j = candidates.lower_bound(&item);
        candidates.insert(j, item);
        candidates.pop();
        println!("{}", candidates[k - 1].1 + 1);
    }
}
