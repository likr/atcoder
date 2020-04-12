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

fn find_components(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = graph.len();
    let mut components = vec![INF; n];
    for w in 0..n {
        if components[w] != INF {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back(w);
        while let Some(u) = queue.pop_front() {
            if components[u] != INF {
                continue;
            }
            components[u] = w;
            for &v in &graph[u] {
                queue.push_back(v);
            }
        }
    }
    components
}

fn dfs(graph: &Vec<Vec<usize>>, u: usize, v: usize, visited: &mut HashSet<usize>) -> bool {
    if visited.contains(&v) {
        return true;
    }
    visited.insert(v);
    for &w in &graph[v] {
        if w != u && dfs(graph, v, w, visited) {
            return true;
        }
    }
    visited.remove(&u);
    false
}

fn has_cycle(graph: &Vec<Vec<usize>>, u: usize) -> bool {
    graph[u].iter().any(|&v| {
        let mut visited = HashSet::new();
        dfs(graph, u, v, &mut visited)
    })
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    let components = find_components(&graph);
    eprintln!("{:?}", components);

    let mut result = 0;
    for u in 0..n {
        if components[u] == u && !has_cycle(&graph, u) {
            result += 1;
        }
    }
    println!("{}", result);
}
