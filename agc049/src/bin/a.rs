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
        s: [Chars; n],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '1' {
                graph[i].push(j);
            }
        }
    }

    let mut distance = vec![vec![INF; n]; n];
    for i in 0..n {
        distance[i][i] = 0;
    }
    for i in 0..n {
        for &j in &graph[i] {
            distance[i][j] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
            }
        }
    }
    let mut result = 0.;
    for i in 0..n {
        let mut count = 0;
        for j in 0..n {
            if distance[j][i] != INF {
                count += 1;
            }
        }
        result += 1. / count as f64;
    }
    println!("{}", result);
}
