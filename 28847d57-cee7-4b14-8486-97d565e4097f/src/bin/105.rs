use ac_library::*;
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
        q: usize,
        abc: [(Usize1, Usize1, i64); m],
        xy: [(Usize1, Usize1); q],
    }
    let mut graph = vec![vec![]; n];
    let mut dsu = Dsu::new(n);
    for &(u, v, c) in abc.iter() {
        dsu.merge(u, v);
        graph[u].push((v, c));
        graph[v].push((u, -c));
    }
    let mut distance = HashMap::new();
    let mut has_cycle = HashSet::new();
    'outer: for s in 0..n {
        if s != dsu.leader(s) {
            continue;
        }
        let mut queue = VecDeque::new();
        distance.insert((s, s), 0);
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            for &(v, c) in graph[u].iter() {
                if !distance.contains_key(&(s, v)) {
                    distance.insert((s, v), distance[&(s, u)] + c);
                    queue.push_back(v);
                } else {
                    if distance[&(s, u)] + c != distance[&(s, v)] {
                        has_cycle.insert(s);
                        continue 'outer;
                    }
                }
            }
        }
    }
    for &(u, v) in xy.iter() {
        if !dsu.same(u, v) {
            println!("nan");
        } else if has_cycle.contains(&dsu.leader(u)) {
            println!("inf");
        } else {
            println!(
                "{}",
                distance[&(dsu.leader(u), v)] - distance[&(dsu.leader(u), u)]
            );
        }
    }
}
