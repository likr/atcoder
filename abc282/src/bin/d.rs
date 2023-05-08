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

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (ui, vi) = uv[i];
        graph[ui].push(vi);
        graph[vi].push(ui);
    }

    let mut group = vec![INF; n];
    let mut height = vec![INF; n];
    for u in 0..n {
        if height[u] == INF {
            let mut queue = VecDeque::new();
            queue.push_back(u);
            height[u] = 0;
            group[u] = u;
            while let Some(v) = queue.pop_front() {
                for &w in graph[v].iter() {
                    if height[w] == INF {
                        queue.push_back(w);
                        height[w] = height[v] + 1;
                        group[w] = u;
                    }
                }
            }
        }
    }
    let mut non_bipartite_group = HashSet::new();
    for i in 0..m {
        let (ui, vi) = uv[i];
        if (height[ui] + height[vi]) % 2 == 0 {
            non_bipartite_group.insert(group[ui]);
        }
    }
    let mut ignore_edge_count = 0;
    for i in 0..m {
        let (ui, vi) = uv[i];
        if non_bipartite_group.contains(&group[ui]) {
            ignore_edge_count += 1;
        }
    }
    let mut even_count = vec![0; n];
    let mut odd_count = vec![0; n];
    for u in 0..n {
        if !non_bipartite_group.contains(&group[u]) {
            if height[u] % 2 == 0 {
                even_count[group[u]] += 1;
            } else {
                odd_count[group[u]] += 1;
            }
        }
    }
}
