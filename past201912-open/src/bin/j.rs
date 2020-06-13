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

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<(usize, Option<usize>)> {
    let mut distance = vec![(INF, None); graph.len()];
    distance[s] = (0, None);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        if distance[u].0 < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u].0 + c < distance[v].0 {
                distance[v] = (distance[u].0 + c, Some(u));
                queue.push(Reverse((distance[v].0, v)));
            }
        }
    }
    distance
}

fn grid_graph(a: &Vec<Vec<usize>>, h: usize, w: usize) -> Vec<Vec<(usize, usize)>> {
    let n = h * w;
    let mut graph = vec![vec![]; n];
    for i in 0..h {
        for j in 1..w {
            let u = i * w + j;
            let v = i * w + j - 1;
            graph[u].push((v, a[i][j - 1]));
            graph[v].push((u, a[i][j]));
        }
    }
    for i in 1..h {
        for j in 0..w {
            let u = i * w + j;
            let v = (i - 1) * w + j;
            graph[u].push((v, a[i - 1][j]));
            graph[v].push((u, a[i][j]));
        }
    }
    graph
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let x = (h - 1) * w;
    let y = h * w - 1;
    let z = w - 1;

    let graph = grid_graph(&a, h, w);
    let mut result = INF;
    for i in 0..h {
        for j in 0..w {
            let distance = dijkstra(&graph, i * w + j);
            let mut vertices = HashSet::new();
            let mut u = x;
            while let Some(v) = distance[u].1 {
                vertices.insert(v);
                u = v;
            }
            let mut u = y;
            while let Some(v) = distance[u].1 {
                vertices.insert(v);
                u = v;
            }
            let mut u = z;
            while let Some(v) = distance[u].1 {
                vertices.insert(v);
                u = v;
            }
            let mut cost = 0;
            for &u in &vertices {
                let i = u / w;
                let j = u % w;
                cost += a[i][j];
            }
            result = min(result, cost);
        }
    }
    println!("{}", result);
}
