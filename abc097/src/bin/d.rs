use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        p: [Usize1; n],
        xy: [(Usize1, Usize1); m],
    }
    let mut union: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
    for &(xi, yi) in &xy {
        union.union(xi, yi);
    }

    let mut count = 0;
    for i in 0..n {
        // eprintln!("{}", union.find(i));
        if union.find(i) == union.find(p[i]) {
            count += 1;
        }
    }
    println!("{}", count);
}
