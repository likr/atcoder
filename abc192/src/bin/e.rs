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
        x: Usize1,
        y: Usize1,
        abtk: [(Usize1, Usize1, usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi, ti, ki) in abtk.iter() {
        graph[ai].push((bi, ti, ki));
        graph[bi].push((ai, ti, ki));
    }
    let mut distance = vec![INF; n];
    distance[x] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, x)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, ti, ki) in graph[u].iter() {
            let c = (ki - d % ki) % ki + ti;
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                heap.push(Reverse((distance[v], v)));
            }
        }
    }
    if distance[y] == INF {
        println!("-1");
    } else {
        println!("{}", distance[y]);
    }
}
