use petgraph::unionfind::*;
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
    let mut k = 0;
    for u in 0..n {
        if components.find(u) == u {
            k += 1;
        }
    }
    println!("{}", k - 1);
}
