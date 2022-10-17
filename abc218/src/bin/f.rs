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

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let mut distance = vec![INF; graph.len()];
    distance[s] = 0;
    let mut parent = vec![None; graph.len()];
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        if distance[u] < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                parent[v] = Some(u);
                queue.push(Reverse((distance[v], v)));
            }
        }
    }
    (distance, parent)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        st: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(si, ti) in st.iter() {
        graph[si].push((ti, 1));
    }
    let (distance, parent) = dijkstra(&graph, 0);
    let mut min_edges = HashSet::new();
    let mut u = n - 1;
    while parent[u].is_some() {
        let v = parent[u].unwrap();
        min_edges.insert((v, u));
        u = v;
    }
    for i in 0..m {
        let (si, ti) = st[i];
        if min_edges.contains(&(si, ti)) {
            let mut graph2 = vec![vec![]; n];
            for j in 0..m {
                if i != j {
                    let (sj, tj) = st[j];
                    graph2[sj].push((tj, 1));
                }
            }
            let (distance2, _) = dijkstra(&graph2, 0);
            if distance2[n - 1] == INF {
                println!("-1");
            } else {
                println!("{}", distance2[n - 1]);
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
