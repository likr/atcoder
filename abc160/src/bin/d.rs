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

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
    }
    let mut graph = vec![vec![]; n];
    for v in 1..n {
        let u = v - 1;
        graph[u].push((v, 1));
        graph[v].push((u, 1));
    }
    graph[x].push((y, 1));
    graph[y].push((x, 1));
    let d = (0..n).map(|u| dijkstra(&graph, u)).collect::<Vec<_>>();
    // eprintln!("{:?}", d);
    let mut count = HashMap::new();
    for i in 0..n {
        for j in 0..i {
            *count.entry(d[i][j]).or_insert(0) += 1;
        }
    }
    for k in 1..n {
        if let Some(c) = count.get(&k) {
            println!("{}", c);
        } else {
            println!("0");
        }
    }
}
