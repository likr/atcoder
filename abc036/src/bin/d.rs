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
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut height = vec![INF; n];
    height[0] = 0;
    let mut queue = VecDeque::new();
    for &u in &graph[0] {
        queue.push_back((0, u));
    }
    while let Some((u, v)) = queue.pop_front() {
        height[v] = height[u] + 1;
        for &w in &graph[v] {
            if height[w] == INF {
                queue.push_back((v, w));
            }
        }
    }

    let mut nodes = (0..n).collect::<Vec<_>>();
    nodes.sort_by_key(|&u| height[u]);
    nodes.reverse();

    let mut dp = vec![(1, 1); n];
    for &u in &nodes {
        for &v in &graph[u] {
            if height[v] < height[u] {
                continue;
            }
            dp[u].0 = dp[u].0 * dp[v].1 % M;
            dp[u].1 = dp[u].1 * ((dp[v].0 + dp[v].1) % M) % M;
        }
    }

    println!("{}", (dp[0].0 + dp[0].1) % M);
}
