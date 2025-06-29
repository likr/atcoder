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
        ab: [(Usize1, Usize1); m],
        q: usize,
        xk: [(Usize1, usize); q],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    for &(x, k) in xk.iter() {
        let mut ans = x + 1;
        let mut distance = HashMap::new();
        let mut queue = VecDeque::new();
        distance.insert(x, 0);
        if k > 0 {
            queue.push_back(x);
        }
        while let Some(u) = queue.pop_front() {
            for &v in graph[u].iter() {
                if !distance.contains_key(&v) {
                    distance.insert(v, distance[&u] + 1);
                    ans += v + 1;
                    if distance[&v] < k {
                        queue.push_back(v);
                    }
                }
            }
        }
        debug!(distance);
        println!("{}", ans);
    }
}
