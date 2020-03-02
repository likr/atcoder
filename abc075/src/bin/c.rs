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
        ab: [(Usize1, Usize1); m],
    }
    let mut count = 0;
    for i in 0..m {
        let mut graph = vec![vec![]; n];
        for j in 0..m {
            if i != j {
                let (aj, bj) = ab[j];
                graph[aj].push(bj);
                graph[bj].push(aj);
            }
        }

        let mut queue = VecDeque::new();
        queue.push_back(0);
        let mut visited = HashSet::new();
        while let Some(u) = queue.pop_front() {
            if visited.contains(&u) {
                continue;
            }
            visited.insert(u);
            for &v in &graph[u] {
                queue.push_back(v);
            }
        }
        if visited.len() < n {
            count += 1;
        }
    }
    println!("{}", count);
}
