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

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ui, vi) in uv.iter() {
        graph[ui].push(vi);
        graph[vi].push(ui);
    }
    let mut height = vec![INF; n];
    let mut root = vec![INF; n];
    for s in 0..n {
        if height[s] != INF {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back(s);
        height[s] = 0;
        root[s] = s;
        while let Some(u) = queue.pop_front() {
            for &v in graph[u].iter() {
                if height[v] == INF {
                    queue.push_back(v);
                    height[v] = height[u] + 1;
                    root[v] = s;
                }
            }
        }
    }

    for u in 0..n {
        for &v in graph[u].iter() {
            if (height[u] + height[v]) % 2 == 0 {
                println!("0");
                return;
            }
        }
    }
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    for u in 0..n {
        if height[u] % 2 == 0 {
            left.entry(root[u]).or_insert(vec![]).push(u);
        } else {
            right.entry(root[u]).or_insert(vec![]).push(u);
        }
    }

    let mut ans = 0;
    for u in 0..n {
        if height[u] % 2 == 0 {
            ans += n - left[&root[u]].len() - graph[u].len();
        } else {
            ans += n - right[&root[u]].len() - graph[u].len();
        }
    }
    println!("{}", ans / 2);
}
