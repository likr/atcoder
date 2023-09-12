use ac_library::*;
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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn dfs(graph: &Vec<Vec<usize>>, u: usize, cache: &mut Vec<usize>) -> usize {
    if cache[u] > 0 {
        return cache[u];
    }
    let mut ans = 1;
    for &v in graph[u].iter() {
        ans = max(ans, dfs(graph, v, cache) + 1);
    }
    cache[u] = ans;
    ans
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let a_max = *a.iter().max().unwrap();
    let mut indices = vec![vec![]; a_max + 1];
    for i in 0..n {
        indices[a[i]].push(i);
    }
    let mut tree = Segtree::<Min<_>>::new(a_max + 1);
    for j in 0..=a_max {
        indices[j].reverse();
        if let Some(&k) = indices[j].last() {
            tree.set(j, k);
        } else {
            tree.set(j, INF);
        }
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        indices[a[i]].pop();
        if let Some(&k) = indices[a[i]].last() {
            tree.set(a[i], k);
        } else {
            tree.set(a[i], INF);
        }
        let s = if a[i] < k { 0 } else { a[i] - k };
        let j = tree.prod(s..a[i]);
        if j < INF {
            graph[j].push(i);
        }
        let t = if a[i] + k > a_max { a_max } else { a[i] + k };
        let j = tree.prod(a[i]..=t);
        if j < INF {
            graph[j].push(i);
        }
    }
    let mut h = vec![0; n];
    let mut ans = 0;
    for u in 0..n {
        ans = max(ans, dfs(&graph, u, &mut h));
    }
    println!("{}", ans);
}
