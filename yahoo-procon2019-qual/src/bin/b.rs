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
        ab: [(Usize1, Usize1); 3],
    }
    let mut degree = vec![0; 4];
    for &(ai, bi) in ab.iter() {
        degree[ai] += 1;
        degree[bi] += 1;
    }
    degree.sort();
    if degree == vec![1, 1, 2, 2] {
        println!("YES");
    } else {
        println!("NO");
    }
}
