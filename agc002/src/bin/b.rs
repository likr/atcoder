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
        xy: [(Usize1, Usize1); m],
    }
    let mut nodes = HashSet::new();
    nodes.insert(0);
    let mut count = vec![1; n];

    for &(xi, yi) in &xy {
        count[xi] -= 1;
        count[yi] += 1;
        if nodes.contains(&xi) {
            nodes.insert(yi);
        }
        if count[xi] == 0 && nodes.contains(&xi) {
            nodes.remove(&xi);
        }
    }
    println!("{}", nodes.len());
}
