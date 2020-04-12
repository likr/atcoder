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
        mut aby: [(Usize1, Usize1, usize); m],
        q: usize,
        vw: [(Usize1, usize); q],
    }
    aby.sort_by_key(|&(_, _, y)| y);
    aby.reverse();
    let mut indices = (0..q).collect::<Vec<_>>();
    indices.sort_by_key(|&i| vw[i].1);
    indices.reverse();

    let mut results = vec![INF; q];
    let mut components: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
    let mut k = 0;
    for &i in &indices {
        while k < m && aby[k].2 > vw[i].1 {
            components.union(aby[k].0, aby[k].1);
            k += 1;
        }
        results[i] = components.get(vw[i].0).size();
    }
    for i in 0..q {
        println!("{}", results[i]);
    }
}
