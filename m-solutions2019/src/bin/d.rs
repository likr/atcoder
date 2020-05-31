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
        ab: [(Usize1, Usize1); n - 1],
        mut c: [usize; n],
    }
    c.sort();

    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let s = c.iter().sum::<usize>() - c[n - 1];

    let mut d = vec![INF; n];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        d[u] = c.pop().unwrap();
        for &v in &graph[u] {
            if d[v] == INF {
                queue.push_back(v);
            }
        }
    }

    let mut t = 0;
    for &(ai, bi) in &ab {
        t += min(d[ai], d[bi]);
    }
    eprintln!("{}", t);

    println!("{}", s);
    for i in 0..n {
        print!("{} ", d[i]);
    }
    println!("");
}
