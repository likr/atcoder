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
        h: usize,
        w: usize,
        ch: Usize1,
        cw: Usize1,
        dh: Usize1,
        dw: Usize1,
        s: [Chars; h],
    }
    let index = |i, j| i * w + j;
    let mut graph = vec![vec![]; h * w];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let u = index(i, j);
            if 1 <= i && s[i - 1][j] != '#' {
                let v = index(i - 1, j);
                graph[u].push((v, 0));
                graph[v].push((u, 0));
            }
            if i + 1 < h && s[i + 1][j] != '#' {
                let v = index(i + 1, j);
                graph[u].push((v, 0));
                graph[v].push((u, 0));
            }
            if 1 <= j && s[i][j - 1] != '#' {
                let v = index(i, j - 1);
                graph[u].push((v, 0));
                graph[v].push((u, 0));
            }
            if j + 1 < w && s[i][j + 1] != '#' {
                let v = index(i, j + 1);
                graph[u].push((v, 0));
                graph[v].push((u, 0));
            }
            for di in 0..5 {
                for dj in 0..5 {
                    if di == 2 && dj == 2 {
                        continue;
                    }
                    if 2 <= i + di
                        && i + di < h + 2
                        && 2 <= j + dj
                        && j + dj < w + 2
                        && s[i + di - 2][j + dj - 2] != '#'
                    {
                        let v = index(i + di - 2, j + dj - 2);
                        graph[u].push((v, 1));
                        graph[v].push((u, 1));
                    }
                }
            }
        }
    }
    // eprintln!("{:?}", graph);
    let distance = dijkstra(&graph, index(ch, cw));
    // eprintln!("{:?}", distance);
    let result = distance[index(dh, dw)];
    if result == INF {
        println!("-1");
    } else {
        println!("{}", distance[index(dh, dw)]);
    }
}
