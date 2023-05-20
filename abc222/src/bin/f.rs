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

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    u: usize,
    dp: &mut Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    parent: &mut Vec<Option<usize>>,
) -> usize {
    let mut s = 0;
    for (i, &(v, c)) in graph[u].iter().enumerate() {
        if !visited[v] {
            visited[v] = true;
            parent[v] = Some(u);
            dp[u][i] = c + dfs(graph, v, dp, visited, parent);
            s = max(s, dp[u][i]);
        }
    }
    s
}

fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, usize); n - 1],
        d: [usize; n],
    }
    let mut graph = vec![vec![]; 2 * n];
    for &(ai, bi, ci) in abc.iter() {
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }
    for i in 0..n {
        graph[i].push((i + n, d[i]));
        graph[i + n].push((i, d[i]));
    }

    let mut ans = vec![0; 2 * n];
    let mut visited = vec![false; 2 * n];
    visited[n] = true;
    let mut parent = vec![None; 2 * n];
    let mut dp = (0..2 * n)
        .map(|u| vec![0; graph[u].len()])
        .collect::<Vec<_>>();
    ans[n] = dfs(&graph, n, &mut dp, &mut visited, &mut parent);
    debug!(parent);

    let mut visited = vec![false; 2 * n];
    visited[0] = true;
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((u, p)) = queue.pop_front() {
        let mut dp_l = vec![0; graph[u].len() + 1];
        let mut dp_r = vec![0; graph[u].len() + 1];
        if let Some(w) = parent[u] {
            for (i, &(v, c)) in graph[u].iter().enumerate() {
                if v == w {
                    dp[u][i] = p;
                    if u % n == v % n {
                        dp[u][i] += d[u % n];
                    } else {
                        dp[u][i] += c;
                    }
                }
            }
        }
        for i in 0..graph[u].len() {
            dp_l[i + 1] = max(dp_l[i], dp[u][i]);
        }
        for i in (0..graph[u].len()).rev() {
            dp_r[i] = max(dp_r[i + 1], dp[u][i]);
        }
        debug!(u, p, dp_l, dp_r);
        ans[u] = dp_r[0];
        for (i, &(v, _)) in graph[u].iter().enumerate() {
            if !visited[v] {
                visited[v] = true;
                queue.push_back((v, max(dp_l[i], dp_r[i + 1])));
            }
        }
    }

    debug!(dp);
    debug!(ans);
    for i in 0..n {
        println!("{}", ans[n + i] - d[i]);
    }
}
