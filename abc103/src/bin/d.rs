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
        mut ab: [(Usize1, Usize1); m],
    }
    let mut c = vec![vec![]; n];
    for &(ai, bi) in &ab {
        c[ai].push(bi);
    }
    let mut result = 0;
    let mut right = HashSet::new();
    for &bi in &c[0] {
        right.insert(bi);
    }
    for i in 1..n {
        if right.contains(&i) {
            result += 1;
            right.clear();
        }
        for &bi in &c[i] {
            right.insert(bi);
        }
    }
    println!("{}", result);
}
