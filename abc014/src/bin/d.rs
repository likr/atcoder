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

fn query(ancestor: &Vec<Vec<usize>>, height: &Vec<usize>, u: usize, v: usize) -> usize {
    let k = ancestor[0].len();
    let (mut u, mut v) = if height[u] < height[v] {
        (v, u)
    } else {
        (u, v)
    };
    for i in 0..k {
        if (height[u] - height[v]) >> i & 1 > 0 {
            u = ancestor[u][i];
        }
    }
    if u == v {
        return u;
    }
    for i in (0..k).rev() {
        if ancestor[u][i] != ancestor[v][i] {
            u = ancestor[u][i];
            v = ancestor[v][i];
        }
    }
    ancestor[u][0]
}

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }
    let mut k = 0;
    let mut m = 1;
    while m < n {
        m *= 2;
        k += 1;
    }

    let mut graph = vec![vec![]; n];
    for &(xi, yi) in &xy {
        graph[xi].push(yi);
        graph[yi].push(xi);
    }

    let mut parent = vec![INF; n];
    parent[0] = 0;
    let mut height = vec![0; n];
    let mut queue = VecDeque::new();
    for &v in &graph[0] {
        queue.push_back((0, v));
    }
    while let Some((u, v)) = queue.pop_front() {
        parent[v] = u;
        height[v] = height[u] + 1;
        for &w in &graph[v] {
            if parent[w] != INF {
                continue;
            }
            queue.push_back((v, w));
        }
    }
    // eprintln!("{:?}", parent);
    // eprintln!("{:?}", height);

    let mut ancestor = vec![vec![INF; k]; n];
    for j in 0..n {
        ancestor[j][0] = parent[j];
    }
    for i in 1..k {
        for j in 0..n {
            ancestor[j][i] = ancestor[ancestor[j][i - 1]][i - 1];
        }
    }
    // for j in 0..n {
    //     eprintln!("{:?}", ancestor[j]);
    // }

    for &(ai, bi) in &ab {
        let w = query(&ancestor, &height, ai, bi);
        // eprintln!("{}", w + 1);
        // eprintln!("{} {} {}", height[ai], height[bi], height[w]);
        println!("{}", height[ai] + height[bi] - 2 * height[w] + 1);
    }
}
