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

fn dfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    x: &HashSet<usize>,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(&c) = memo.get(&u) {
        c
    } else {
        let mut c = 0;
        if x.contains(&u) {
            c += 1;
        }
        for &v in graph[u].iter() {
            c += dfs(graph, v, x, memo);
        }
        memo.insert(u, c);
        c
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(Usize1, Usize1); n - 1],
        v: [Usize1; k],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut parent = vec![None; n];
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    let s = v[0];
    visited[s] = true;
    queue.push_back(s);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if !visited[v] {
                parent[v] = Some(u);
                queue.push_back(v);
                visited[v] = true;
            }
        }
    }
    let mut children = vec![vec![]; n];
    for u in 0..n {
        if let Some(v) = parent[u] {
            children[v].push(u);
        }
    }
    let x = v.clone().into_iter().collect::<HashSet<usize>>();
    let mut memo = HashMap::new();
    let mut ans = n;
    for u in 0..n {
        if dfs(&children, u, &x, &mut memo) == 0 {
            ans -= 1;
        }
    }
    println!("{}", ans);
}
