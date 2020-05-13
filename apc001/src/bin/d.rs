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
        a: [usize; n],
        xy: [(usize, usize); m],
    }

    let mut components: QuickFindUf<UnionBySize> = QuickFindUf::new(n);
    for &(xi, yi) in &xy {
        components.union(xi, yi);
    }

    let mut nodes = vec![vec![]; n];
    for u in 0..n {
        nodes[components.find(u)].push(u);
    }

    let mut n_components = 0;
    let mut result = 0usize;
    let mut visited = vec![false; n];

    for u in 0..n {
        if u == components.find(u) {
            n_components += 1;
            let v = *nodes[u].iter().min_by_key(|&&v| a[v]).unwrap();
            result += a[v];
            visited[v] = true;
        }
    }

    if n_components == 1 {
        println!("0");
    } else {
        let m = n_components - 2;
        let mut b = (0..n)
            .filter(|&i| !visited[i])
            .map(|i| a[i])
            .collect::<Vec<_>>();
        if b.len() < m {
            println!("Impossible");
        } else {
            b.sort();
            for i in 0..m {
                result += b[i];
            }
            println!("{}", result);
        }
    }
}
