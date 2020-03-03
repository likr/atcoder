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
        k: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); k],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut components = vec![INF; n];
    for w in 0..n {
        if components[w] != INF {
            continue;
        }
        let mut queue = VecDeque::new();
        queue.push_back(w);
        while let Some(u) = queue.pop_front() {
            if components[u] != INF {
                continue;
            }
            components[u] = w;
            for &v in &graph[u] {
                queue.push_back(v);
            }
        }
    }
    let mut components_set = HashMap::new();
    for u in 0..n {
        components_set
            .entry(components[u])
            .or_insert(HashSet::new())
            .insert(u);
    }
    // eprintln!("{:?}", components_set);
    for &(ci, di) in &cd {
        if components[ci] == components[di] {
            graph[ci].push(di);
            graph[di].push(ci);
        }
    }
    for u in 0..n {
        print!(
            "{} ",
            components_set[&components[u]].len() - graph[u].len() - 1
        );
    }
    println!("");
}
