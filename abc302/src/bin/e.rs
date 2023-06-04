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
        q: usize,
    }
    let mut count = n;
    let mut degree = vec![0; n];
    let mut neighbors = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            }
            if degree[u] == 0 {
                count -= 1;
            }
            if degree[v] == 0 {
                count -= 1;
            }
            degree[u] += 1;
            degree[v] += 1;
            neighbors[u].insert(v);
            neighbors[v].insert(u);
        } else {
            input! {
                u: Usize1,
            }
            if degree[u] > 0 {
                count += 1;
            }
            degree[u] = 0;
            let vs = neighbors[u].clone();
            for &v in vs.iter() {
                degree[v] -= 1;
                if degree[v] == 0 {
                    count += 1;
                }
                neighbors[v].remove(&u);
            }
            neighbors[u].clear();
        }
        println!("{}", count);
    }
}
