use petgraph::unionfind::UnionFind;
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
    let mut components = UnionFind::new(n);
    for &(ai, bi) in &ab {
        components.union(ai, bi);
    }
    let mut size = HashMap::new();
    for i in 0..n {
        *size.entry(components.find(i)).or_insert(0) += 1usize;
    }
    println!("{}", size.values().max().unwrap());
}
