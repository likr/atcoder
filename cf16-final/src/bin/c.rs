use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        l: [[Usize1]; n],
    }

    let mut graph = vec![vec![]; n + m];
    for i in 0..n {
        for j in 0..l[i].len() {
            graph[i].push(l[i][j] + n);
            graph[l[i][j] + n].push(i);
        }
    }
    // eprintln!("{:?}", graph);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        if visited.contains(&u) {
            continue;
        }
        visited.insert(u);
        for &v in &graph[u] {
            queue.push_back(v);
        }
    }
    // eprintln!("{:?}", visited);

    if (0..n).all(|i| visited.contains(&i)) {
        println!("YES");
    } else {
        println!("NO");
    }
}
