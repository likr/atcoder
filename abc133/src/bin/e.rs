use proconio::input;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    k: usize,
    used: usize,
    f: &Vec<usize>,
    visited: &mut HashSet<usize>,
    count: &mut usize,
) {
    let c = f[k - used] * inv(f[k - graph[u].len() - 1]) % M;
    *count = *count * c % M;
    visited.insert(u);
    for &v in &graph[u] {
        if !visited.contains(&v) {
            dfs(graph, v, k, 2, f, visited, count);
        }
    }
}

fn main() {
    input! {
      n: usize,
      k: usize,
      ab: [(usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n + 1];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let max = graph.iter().map(|row| row.len() + 1).max().unwrap();
    if k < max {
        println!("0");
        return;
    }

    let mut f = vec![1; k + 1];
    for i in 2..=k {
        f[i] = i * f[i - 1] % M;
    }

    let mut visited = HashSet::new();
    let mut count = 1;
    dfs(&graph, 1, k, 0, &f, &mut visited, &mut count);
    println!("{}", count);
}
