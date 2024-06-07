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
        p: [Usize1; n - 1],
        xy: [(Usize1, usize); m],
    }
    let mut children = vec![vec![]; n];
    for i in 1..n {
        children[p[i - 1]].push(i);
    }
    let mut values = vec![0; n];
    for &(xi, yi) in xy.iter() {
        values[xi] = max(values[xi], yi + 1);
    }
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in children[u].iter() {
            if values[u] > 0 {
                values[v] = max(values[v], values[u] - 1);
            }
            queue.push_back(v);
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if values[i] > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
