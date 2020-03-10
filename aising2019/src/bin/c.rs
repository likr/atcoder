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
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let n = h * w;
    let mut graph = vec![vec![]; n];
    for i in 0..h {
        for j in 1..w {
            if s[i][j - 1] != s[i][j] {
                graph[w * i + j - 1].push(w * i + j);
                graph[w * i + j].push(w * i + j - 1);
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            if s[i - 1][j] != s[i][j] {
                graph[w * (i - 1) + j].push(w * i + j);
                graph[w * i + j].push(w * (i - 1) + j);
            }
        }
    }

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
    // eprintln!("{:?}", components);

    let mut component_nodes = vec![vec![]; n];
    for u in 0..n {
        component_nodes[components[u]].push(u);
    }

    let mut result = 0usize;
    for c in 0..n {
        let mut black = 0;
        let mut white = 0;
        for &u in &component_nodes[c] {
            if s[u / w][u % w] == '#' {
                black += 1;
            } else {
                white += 1;
            }
        }
        result += black * white;
    }
    println!("{}", result);
}
