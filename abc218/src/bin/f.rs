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
        st: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(si, ti) in st.iter() {
        graph[si].push(ti);
    }
    let mut distance = vec![INF; n];
    let mut parent = vec![INF; n];
    let mut queue = VecDeque::new();
    distance[0] = 0;
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                parent[v] = u;
                queue.push_back(v);
            }
        }
    }
    let mut edges = HashSet::new();
    let mut v = n - 1;
    let mut u = parent[n - 1];
    while u != INF {
        edges.insert((u, v));
        edges.insert((v, u));
        v = u;
        u = parent[v];
    }
    for i in 0..m {
        let (si, ti) = st[i];
        if edges.contains(&(si, ti)) {
            let mut graph = vec![vec![]; n];
            for j in 0..m {
                if i != j {
                    let (sj, tj) = st[j];
                    graph[sj].push(tj);
                }
            }
            let mut distance = vec![INF; n];
            let mut parent = vec![INF; n];
            let mut queue = VecDeque::new();
            distance[0] = 0;
            queue.push_back(0);
            while let Some(u) = queue.pop_front() {
                for &v in graph[u].iter() {
                    if distance[v] == INF {
                        distance[v] = distance[u] + 1;
                        parent[v] = u;
                        queue.push_back(v);
                    }
                }
            }
            if distance[n - 1] == INF {
                println!("-1");
            } else {
                println!("{}", distance[n - 1]);
            }
        } else {
            if distance[n - 1] == INF {
                println!("-1");
            } else {
                println!("{}", distance[n - 1]);
            }
        }
    }
}
