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
    let mut m = n;
    let mut graph = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                u: Usize1,
                v: Usize1,
            }
            if graph[u].len() == 0 {
                m -= 1;
            }
            if graph[v].len() == 0 {
                m -= 1;
            }
            graph[u].insert(v);
            graph[v].insert(u);
        } else {
            input! {
                v: Usize1,
            }
            let s = graph[v].clone();
            for &u in s.iter() {
                graph[u].remove(&v);
                if graph[u].len() == 0 {
                    m += 1;
                }
            }
            if graph[v].len() > 0 {
                m += 1;
            }
            graph[v].clear();
        }
        println!("{}", m);
    }
}
