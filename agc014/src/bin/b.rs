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
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut count = vec![0; n];
    for &(ai, bi) in &ab {
        count[ai] += 1;
        count[bi] += 1;
    }
    if (0..n).all(|i| count[i] % 2 == 0) {
        println!("YES");
    } else {
        println!("NO");
    }
}
