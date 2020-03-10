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
    let mut x_indices = (0..n).collect::<Vec<usize>>();
    x_indices.sort_by_key(|&i| xy[i].0);
    let mut y_indices = (0..n).collect::<Vec<usize>>();
    y_indices.sort_by_key(|&i| xy[i].1);

    let mut graph = vec![vec![]; n];
    for k in 1..n {
        let i = x_indices[k - 1];
        let (xi, yi) = xy[i];
        let j = x_indices[k];
        let (xj, yj) = xy[j];
        let d = min((xi - xj).abs(), (yi - yj).abs());
        graph[i].push((j, d));
        graph[j].push((i, d));
    }
    for k in 1..n {
        let i = y_indices[k - 1];
        let (xi, yi) = xy[i];
        let j = y_indices[k];
        let (xj, yj) = xy[j];
        let d = min((xi - xj).abs(), (yi - yj).abs());
        graph[i].push((j, d));
        graph[j].push((i, d));
    }

    // for u in 0..n {
    //     eprintln!("{:?}", graph[u]);
    // }

    let mut heap = BinaryHeap::new();
    for u in 0..n {
        for &(v, d) in &graph[u] {
            heap.push(Reverse((d, u, v)));
        }
    }
    // eprintln!("{:?}", heap);
    let mut cost = 0;
    let mut components: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
    while let Some(Reverse((d, u, v))) = heap.pop() {
        if components.find(u) == components.find(v) {
            continue;
        }
        // eprintln!("{} {}", u, v);
        components.union(u, v);
        cost += d;
    }
    println!("{}", cost);
}
