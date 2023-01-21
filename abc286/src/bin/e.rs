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
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                graph[i].push(j);
            }
        }
    }
    let mut distance = vec![vec![INF; n]; n];
    let mut value = vec![vec![0; n]; n];
    for s in 0..n {
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, Reverse(a[s]), s)));
        distance[s][s] = 0;
        value[s][s] = a[s];
        while let Some(Reverse((d, Reverse(c), u))) = queue.pop() {
            if (distance[s][u], Reverse(value[s][u])) < (d, Reverse(c)) {
                continue;
            }
            for &v in graph[u].iter() {
                if (distance[s][u] + 1, Reverse(value[s][u] + a[v]))
                    < (distance[s][v], Reverse(value[s][v]))
                {
                    distance[s][v] = distance[s][u] + 1;
                    value[s][v] = value[s][u] + a[v];
                    queue.push(Reverse((distance[s][v], Reverse(value[s][v]), v)));
                }
            }
        }
    }
    for &(ui, vi) in uv.iter() {
        if distance[ui][vi] == INF {
            println!("Impossible");
        } else {
            println!("{} {}", distance[ui][vi], value[ui][vi]);
        }
    }
}
