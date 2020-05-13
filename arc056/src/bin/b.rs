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

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut f = HashSet::new();
    f.insert(s);

    let mut result = vec![];
    for u in (0..n).rev() {
        if f.contains(&u) {
            result.push(u);

            let mut queue = VecDeque::new();
            queue.push_back(u);
            while let Some(v) = queue.pop_front() {
                for &w in &graph[v] {
                    if w > u && !f.contains(&w) {
                        queue.push_back(w);
                    }
                    f.insert(w);
                }
            }
        }
    }
    result.reverse();

    for &u in &result {
        println!("{}", u + 1);
    }
}
