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

fn query(ancestor: &Vec<Vec<Option<usize>>>, height: &Vec<usize>, u: usize, v: usize) -> usize {
    let k = ancestor[0].len();
    let (mut u, mut v) = if height[u] < height[v] {
        (v, u)
    } else {
        (u, v)
    };
    for i in 0..k {
        if (height[u] - height[v]) >> i & 1 > 0 {
            if let Some(w) = ancestor[u][i] {
                u = w;
            }
        }
    }
    if u == v {
        return u;
    }
    for i in (0..k).rev() {
        if ancestor[u][i] != ancestor[v][i] {
            u = ancestor[u][i].unwrap();
            v = ancestor[v][i].unwrap();
        }
    }
    ancestor[u][0].unwrap()
}

fn main() {
    input! {
        n: usize,
        p: [Isize1; n],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }
    let p = p
        .into_iter()
        .map(|pi| if pi < 0 { None } else { Some(pi as usize) })
        .collect::<Vec<_>>();

    let mut root = INF;
    let mut graph = vec![vec![]; n];
    for u in 0..n {
        if let Some(v) = p[u] {
            graph[v].push(u);
        } else {
            root = u;
        }
    }

    let mut height = vec![INF; n];
    height[root] = 0;
    let mut queue = VecDeque::new();
    for &v in &graph[root] {
        queue.push_back((root, v));
    }
    while let Some((u, v)) = queue.pop_front() {
        height[v] = height[u] + 1;
        for &w in &graph[v] {
            queue.push_back((v, w));
        }
    }

    let k = {
        let mut k = 0;
        let mut m = 1;
        while m < n {
            m *= 2;
            k += 1;
        }
        k
    };
    let mut ancestor = vec![vec![None; k]; n];
    for j in 0..n {
        if let Some(pj) = p[j] {
            ancestor[j][0] = Some(pj);
        }
    }
    for i in 1..k {
        for j in 0..n {
            if let Some(p) = ancestor[j][i - 1] {
                if let Some(q) = ancestor[p][i - 1] {
                    ancestor[j][i] = Some(q);
                }
            }
        }
    }

    for &(ai, bi) in &ab {
        let ci = query(&ancestor, &height, ai, bi);
        if bi == ci {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
