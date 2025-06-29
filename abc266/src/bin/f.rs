use ac_library::Dsu;
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
        uv: [(Usize1, Usize1); n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }
    let mut graph = vec![vec![]; n];
    for &(ui, vi) in uv.iter() {
        graph[ui].push(vi);
        graph[vi].push(ui);
    }
    let mut queue = VecDeque::new();
    let mut distance = vec![INF; n];
    let mut parent = vec![None; n];
    let mut list1 = vec![];
    let mut list2 = vec![];
    queue.push_back(0);
    distance[0] = 0;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                parent[v] = Some(u);
                queue.push_back(v);
            } else if Some(v) != parent[u] {
                list1.push(u);
                list2.push(v);
            }
        }
    }
    debug!(list1, list2);
    let mut p = parent[list1[0]];
    while let Some(u) = p {
        list1.push(u);
        p = parent[u];
    }
    let mut p = parent[list2[0]];
    while let Some(u) = p {
        list2.push(u);
        p = parent[u];
    }
    let mut w = 0;
    while list1.last().unwrap() == list2.last().unwrap() {
        w = list1.pop().unwrap();
        list2.pop().unwrap();
    }
    let mut cycle_nodes = HashSet::new();
    cycle_nodes.insert(w);
    for &u in list1.iter() {
        cycle_nodes.insert(u);
    }
    for &u in list2.iter() {
        cycle_nodes.insert(u);
    }

    let mut dsu = Dsu::new(n);
    for &(ui, vi) in uv.iter() {
        if !cycle_nodes.contains(&ui) || !cycle_nodes.contains(&vi) {
            dsu.merge(ui, vi);
        }
    }
    for &(xi, yi) in xy.iter() {
        if dsu.same(xi, yi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
