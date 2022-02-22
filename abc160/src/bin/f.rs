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

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn dfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    visited: &mut HashSet<usize>,
    dp: &mut Vec<HashMap<usize, (usize, usize)>>,
    f: &Vec<usize>,
    size: &Vec<usize>,
) -> usize {
    let mut result = f[size[u] - 1];
    for &v in graph[u].iter() {
        if !visited.contains(&v) {
            visited.insert(v);
            let x = dfs(graph, v, visited, dp, f, size);
            dp[u].insert(v, (x, size[v]));
            result = (result * dp[u][&v].0) % M;
            result = (result * inv(f[size[v]])) % M;
        }
    }
    result
}

fn bfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    p: usize,
    dp: &mut Vec<HashMap<usize, (usize, usize)>>,
    result: &mut Vec<usize>,
    f: &Vec<usize>,
    size: &Vec<usize>,
) {
    if let Some(&w) = graph[u].iter().find(|&&v| !dp[u].contains_key(&v)) {
        dp[u].insert(w, (p, graph.len() - size[u]));
    }
    let d = graph[u].len();
    let mut left = vec![];
    left.push(1);
    for i in 1..=d {
        let (x, y) = dp[u][&graph[u][i - 1]];
        let v = (left[i - 1] * x) % M;
        left.push(v * inv(f[y]) % M);
    }
    let mut right = vec![];
    right.push(1);
    for i in (0..d).rev() {
        let (x, y) = dp[u][&graph[u][i]];
        let v = (right.last().unwrap() * x) % M;
        right.push(v * inv(f[y]) % M);
    }
    right.reverse();
    result[u] = (f[graph.len() - 1] * left[d]) % M;
    for i in 0..d {
        let v = graph[u][i];
        if result[v] == INF {
            bfs(
                graph,
                v,
                f[graph.len() - size[v] - 1] * (left[i] * right[i + 1] % M) % M,
                dp,
                result,
                f,
                size,
            );
        }
    }
}

fn count_size(
    graph: &Vec<Vec<usize>>,
    u: usize,
    visited: &mut HashSet<usize>,
    size: &mut Vec<usize>,
) -> usize {
    let mut result = 1;
    for &v in graph[u].iter() {
        if !visited.contains(&v) {
            visited.insert(v);
            result += count_size(graph, v, visited, size);
        }
    }
    size[u] = result;
    result
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut f = vec![1; n + 1];
    for i in 1..=n {
        f[i] = (f[i - 1] * i) % M;
    }

    let mut visited = HashSet::new();
    visited.insert(0);
    let mut size = vec![0; n];
    count_size(&graph, 0, &mut visited, &mut size);

    let mut dp = vec![];
    for _ in 0..n {
        dp.push(HashMap::new());
    }
    let mut visited = HashSet::new();
    visited.insert(0);
    dfs(&graph, 0, &mut visited, &mut dp, &f, &size);
    let mut result = vec![INF; n];
    bfs(&graph, 0, result[0], &mut dp, &mut result, &f, &size);

    for u in 0..n {
        println!("{}", result[u]);
    }
}
