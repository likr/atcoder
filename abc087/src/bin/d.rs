use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
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

fn dfs(graph: &Vec<Vec<(usize, i64)>>, u: usize) -> Option<i64> {
    if graph[u].is_empty() {
        return Some(0);
    }
    let mut x = None;
    for &(v, d) in &graph[u] {
        match dfs(graph, v) {
            Some(xv) => match x {
                Some(xu) => {
                    if xu != xv - d {
                        return None;
                    }
                }
                None => {
                    x = Some(xv - d);
                }
            },
            None => {
                return None;
            }
        }
    }
    x
}

fn find_cycle(graph: &Vec<Vec<(usize, i64)>>, u: usize, visited: &mut HashSet<usize>, path: &mut HashSet<usize>) -> bool {
    if visited.contains(&u) {
        return false;
    }
    if path.contains(&u) {
        return true;
    }
    visited.insert(u);
    !graph[u]
        .iter()
        .all(|&(v, _)| !find_cycle(graph, v, visited))
}

fn main() {
    input! {
      n: usize,
      m: usize,
      lrd: [(Usize1, Usize1, i64); m],
    }
    let mut in_degree = vec![0; n];
    let mut graph = vec![vec![]; n];
    for &(li, ri, di) in &lrd {
        graph[li].push((ri, di));
        in_degree[ri] += 1;
    }

    let mut visited = HashSet::new();
    for u in 0..n {
        if find_cycle(
    }

    let mut check = false;
    for u in 0..n {
        if in_degree[u] == 0 {
            println!("{}", u);
            check = true;
            if find_cycle(&graph, u, &mut visited) || dfs(&graph, u) == None {
                println!("No");
                return;
            }
        }
    }
    if check {
        println!("Yes");
    } else {
        println!("No");
    }
}
