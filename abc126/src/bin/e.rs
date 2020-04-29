use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use union_find::{QuickFindUf, UnionBySize, UnionFind};

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(Usize1, Usize1, usize); m],
    }

    let mut components: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
    for &(xi, yi, _) in &xyz {
        components.union(xi, yi);
    }
    let mut result = 0;
    for u in 0..n {
        if u == components.find(u) {
            result += 1;
        }
    }
    println!("{}", result);
}
