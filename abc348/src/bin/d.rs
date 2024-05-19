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

fn bfs(s: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = graph.len();
    let mut queue = VecDeque::new();
    let mut distance = vec![INF; n];
    queue.push_back(s);
    distance[s] = 0;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                queue.push_back(v);
                distance[v] = distance[u] + 1;
            }
        }
    }
    distance
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a0: [Chars; h],
        n: usize,
        rce: [(usize, usize, usize); n],
    }
    let w = w + 2;
    let h = h + 2;
    let mut a = vec![vec!['#'; w]; h];
    let mut s = (0, 0);
    let mut t = (0, 0);
    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if a0[i - 1][j - 1] == 'S' {
                s = (i, j);
                a[i][j] = '.';
            } else if a0[i - 1][j - 1] == 'T' {
                t = (i, j);
                a[i][j] = '.';
            } else {
                a[i][j] = a0[i - 1][j - 1];
            }
        }
    }

    let mut graph = vec![vec![]; h * w];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '.' && a[i + 1][j] == '.' {
                let u = i * w + j;
                let v = (i + 1) * w + j;
                graph[u].push(v);
                graph[v].push(u);
            }
            if a[i][j] == '.' && a[i][j + 1] == '.' {
                let u = i * w + j;
                let v = i * w + j + 1;
                graph[u].push(v);
                graph[v].push(u);
            }
        }
    }

    let mut nodes = vec![];
    nodes.push((s.0 * w + s.1, 0));
    for i in 0..n {
        let (ri, ci, ei) = rce[i];
        nodes.push((ri * w + ci, ei));
    }
    nodes.push((t.0 * w + t.1, 0));
    let mut graph2 = vec![vec![]; nodes.len()];
    for (i, &(u, eu)) in nodes.iter().enumerate() {
        let d_u = bfs(u, &graph);
        for (j, &(v, _)) in nodes.iter().enumerate() {
            if i != j && d_u[v] <= eu {
                graph2[i].push(j);
            }
        }
    }
    let ans = bfs(0, &graph2);
    if *ans.last().unwrap() < INF {
        println!("Yes");
    } else {
        println!("No");
    }
}
