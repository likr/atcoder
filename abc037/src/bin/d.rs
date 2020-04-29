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
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let n = h * w;
    let mut out_nodes = vec![vec![]; n];
    let mut in_nodes = vec![vec![]; n];
    for i in 1..h {
        for j in 0..w {
            let u = (i - 1) * w + j;
            let v = i * w + j;
            if a[i - 1][j] > a[i][j] {
                out_nodes[v].push(u);
                in_nodes[u].push(v);
            } else if a[i - 1][j] < a[i][j] {
                out_nodes[u].push(v);
                in_nodes[v].push(u);
            }
        }
    }
    for i in 0..h {
        for j in 1..w {
            let u = i * w + j - 1;
            let v = i * w + j;
            if a[i][j - 1] > a[i][j] {
                out_nodes[v].push(u);
                in_nodes[u].push(v);
            } else if a[i][j - 1] < a[i][j] {
                out_nodes[u].push(v);
                in_nodes[v].push(u);
            }
        }
    }

    let mut ancestors = VecDeque::new();
    for u in 0..n {
        if in_nodes[u].is_empty() {
            ancestors.push_back(u);
        }
    }
    let mut sorted = vec![];
    while let Some(u) = ancestors.pop_front() {
        sorted.push(u);
        for &v in &out_nodes[u] {
            let k = in_nodes[v].iter().position(|&w| w == u).unwrap();
            in_nodes[v].remove(k);
            if in_nodes[v].is_empty() {
                ancestors.push_back(v);
            }
        }
        out_nodes[u].clear();
    }

    for i in 1..h {
        for j in 0..w {
            let u = (i - 1) * w + j;
            let v = i * w + j;
            if a[i - 1][j] > a[i][j] {
                out_nodes[v].push(u);
                in_nodes[u].push(v);
            } else if a[i - 1][j] < a[i][j] {
                out_nodes[u].push(v);
                in_nodes[v].push(u);
            }
        }
    }
    for i in 0..h {
        for j in 1..w {
            let u = i * w + j - 1;
            let v = i * w + j;
            if a[i][j - 1] > a[i][j] {
                out_nodes[v].push(u);
                in_nodes[u].push(v);
            } else if a[i][j - 1] < a[i][j] {
                out_nodes[u].push(v);
                in_nodes[v].push(u);
            }
        }
    }

    let mut count = vec![1; n];
    for &u in &sorted {
        for &v in &out_nodes[u] {
            count[v] = (count[v] + count[u]) % M;
        }
    }
    // eprintln!("{:?}", count);
    let mut s = 0usize;
    for &c in &count {
        s = (s + c) % M;
    }

    println!("{}", s);
}
