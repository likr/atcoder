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
        k: usize,
        l: usize,
        pq: [(Usize1, Usize1); k],
        rs: [(Usize1, Usize1); l],
    }

    let mut components1: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
    for &(pi, qi) in &pq {
        components1.union(pi, qi);
    }

    let mut components2: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
    for &(ri, si) in &rs {
        components2.union(ri, si);
    }

    let mut count = HashMap::new();
    for u in 0..n {
        let c1 = components1.find(u);
        let c2 = components2.find(u);
        *count.entry((c1, c2)).or_insert(0) += 1;
    }

    for u in 0..n {
        let c1 = components1.find(u);
        let c2 = components2.find(u);
        println!("{}", count[&(c1, c2)]);
    }
}
