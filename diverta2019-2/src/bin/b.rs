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
        xy: [(isize, isize); n],
    }
    let mut pq = HashMap::new();
    for i in 0..n {
        let (xi, yi) = xy[i];
        for j in 0..i {
            if i != j {
                let (xj, yj) = xy[j];
                pq.entry((xi - xj, yi - yj)).or_insert(vec![]).push((i, j));
                pq.entry((xj - xi, yj - yi)).or_insert(vec![]).push((j, i));
            }
        }
    }
    let mut result = n;
    for pairs in pq.values() {
        let mut components: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
        for &(i, j) in pairs {
            components.union(i, j);
        }
        let mut c = 0;
        for i in 0..n {
            if components.find(i) == i {
                c += 1;
            }
        }
        result = min(result, c);
    }
    println!("{}", result);
}
