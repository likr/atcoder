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

fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(s);
    let mut distance = vec![INF; graph.len()];
    distance[s] = 0;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                queue.push_back(v);
            }
        }
    }
    distance
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut s = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                s = (i, j);
            }
        }
    }

    let mut graph = vec![vec![]; h * w];
    for i in 1..h {
        for j in 0..w {
            if c[i - 1][j] == '.' && c[i][j] == '.' {
                graph[(i - 1) * w + j].push(i * w + j);
                graph[(i) * w + j].push((i - 1) * w + j);
            }
        }
    }
    for i in 0..h {
        for j in 1..w {
            if c[i][j - 1] == '.' && c[i][j] == '.' {
                graph[(i) * w + j - 1].push(i * w + j);
                graph[(i) * w + j].push((i) * w + j - 1);
            }
        }
    }

    if s.0 > 0 && c[s.0 - 1][s.1] != '#' {
        let distance = bfs(&graph, (s.0 - 1) * w + s.1);
        if s.0 + 1 < h
            && c[s.0 + 1][s.1] != '#'
            && distance[(s.0 + 1) * w + s.1] != INF
            && distance[(s.0 + 1) * w + s.1] >= 2
        {
            println!("Yes");
            return;
        }
        if s.1 > 0
            && c[s.0][s.1 - 1] != '#'
            && distance[(s.0) * w + (s.1 - 1)] != INF
            && distance[(s.0) * w + (s.1 - 1)] >= 2
        {
            println!("Yes");
            return;
        }
        if s.1 + 1 < w
            && c[s.0][s.1 + 1] != '#'
            && distance[s.0 * w + s.1 + 1] != INF
            && distance[s.0 * w + s.1 + 1] >= 2
        {
            println!("Yes");
            return;
        }
    }
    if s.0 + 1 < h && c[s.0 + 1][s.1] != '#' {
        let distance = bfs(&graph, (s.0 + 1) * w + s.1);
        if s.0 > 0
            && c[s.0 - 1][s.1] != '#'
            && distance[(s.0 - 1) * w + s.1] != INF
            && distance[(s.0 - 1) * w + s.1] >= 2
        {
            println!("Yes");
            return;
        }
        if s.1 > 0
            && c[s.0][s.1 - 1] != '#'
            && distance[(s.0) * w + (s.1 - 1)] != INF
            && distance[(s.0) * w + (s.1 - 1)] >= 2
        {
            println!("Yes");
            return;
        }
        if s.1 + 1 < w
            && c[s.0][s.1 + 1] != '#'
            && distance[s.0 * w + s.1 + 1] != INF
            && distance[s.0 * w + s.1 + 1] >= 2
        {
            println!("Yes");
            return;
        }
    }
    if s.1 > 0 && c[s.0][s.1 - 1] != '#' {
        let distance = bfs(&graph, (s.0) * w + s.1 - 1);
        if s.0 > 0
            && c[s.0 - 1][s.1] != '#'
            && distance[(s.0 - 1) * w + s.1] != INF
            && distance[(s.0 - 1) * w + s.1] >= 2
        {
            println!("Yes");
            return;
        }
        if s.0 + 1 < h
            && c[s.0 + 1][s.1] != '#'
            && distance[(s.0 + 1) * w + s.1] != INF
            && distance[(s.0 + 1) * w + s.1] >= 2
        {
            println!("Yes");
            return;
        }
        if s.1 + 1 < w
            && c[s.0][s.1 + 1] != '#'
            && distance[(s.0) * w + s.1 + 1] != INF
            && distance[s.0 * w + s.1 + 1] >= 2
        {
            println!("Yes");
            return;
        }
    }
    if s.1 + 1 < w && c[s.0][s.1 + 1] != '#' {
        let distance = bfs(&graph, (s.0) * w + s.1 + 1);
        if s.0 > 0
            && c[s.0 - 1][s.1] != '#'
            && distance[(s.0 - 1) * w + s.1] != INF
            && distance[(s.0 - 1) * w + s.1] >= 2
        {
            println!("Yes");
            return;
        }
        if s.0 + 1 < h
            && c[s.0 + 1][s.1] != '#'
            && distance[(s.0 + 1) * w + s.1] != INF
            && distance[(s.0 + 1) * w + s.1] >= 2
        {
            println!("Yes");
            return;
        }
        if s.1 > 0
            && c[s.0][s.1 - 1] != '#'
            && distance[(s.0) * w + (s.1 - 1)] != INF
            && distance[(s.0) * w + (s.1 - 1)] >= 2
        {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
