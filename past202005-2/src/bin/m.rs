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

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let mut distance = vec![INF; graph.len()];
    distance[s] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        if distance[u] < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                queue.push(Reverse((distance[v], v)));
            }
        }
    }
    distance
}

fn rec(dp: &mut Vec<Vec<usize>>, distance: &Vec<Vec<usize>>, state: usize, u: usize, n: usize) {
    if dp[state][u] != INF {
        return;
    }
    for v in 0..n {
        if state & 1 << v == 0 {
            rec(dp, distance, state | 1 << v, v, n);
            dp[state][u] = min(dp[state][u], dp[state | 1 << v][v] + distance[u][v]);
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        s: Usize1,
        k: usize,
        mut t: [Usize1; k],
    }
    t.reverse();
    t.push(s);
    t.reverse();
    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push((v, 1));
        graph[v].push((u, 1));
    }

    let mut distance2 = vec![vec![0; k + 1]; k + 1];
    for i in 0..=k {
        let distance = dijkstra(&graph, t[i]);
        for j in 0..=k {
            distance2[i][j] = distance[t[j]];
            distance2[j][i] = distance[t[j]];
        }
    }

    let mut dp = vec![vec![INF; k + 1]; 2usize.pow(k as u32 + 1)];
    let x = dp.len() - 1;
    for i in 0..k + 1 {
        dp[x][i] = 0;
    }
    rec(&mut dp, &distance2, 0, 0, k + 1);
    // for s in 0..dp.len() {
    //     eprintln!("{} {:?}", s, dp[s]);
    // }
    println!("{}", dp[0][0]);
}
