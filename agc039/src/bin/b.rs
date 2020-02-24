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
        s: [Chars; n],
    }

    let mut color = vec![0; n];
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((u, v)) = queue.pop_front() {
        if color[v] != 0 {
            continue;
        }
        color[v] = if color[u] == 1 { 2 } else { 1 };
        for w in 0..n {
            if s[v][w] == '1' {
                queue.push_back((v, w));
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '1' && color[i] == color[j] {
                println!("-1");
                return;
            }
        }
    }

    let mut distance = vec![vec![INF; n]; n];
    for i in 0..n {
        distance[i][i] = 0;
    }
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '1' {
                distance[i][j] = 1;
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
            }
        }
    }

    let mut max_distance = 0;
    for i in 0..n {
        for j in 0..n {
            if distance[i][j] != INF {
                max_distance = max(max_distance, distance[i][j]);
            }
        }
    }

    println!("{}", max_distance + 1);
}
