use petgraph::unionfind::*;
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
        mut abc: [(Usize1, Usize1, usize); m],
    }
    let mut components = UnionFind::new(n);
    let mut cost = vec![INF; m];
    let mut s = 0;
    let mut indices = (0..m).collect::<Vec<_>>();
    indices.sort_by_key(|&i| abc[i].2);
    for &i in &indices {
        let (ai, bi, ci) = abc[i];
        if components.union(ai, bi) {
            s += ci;
            cost[i] = 0;
        }
    }

    let mut edges = HashMap::new();
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        if cost[i] == 0 {
            let (ai, bi, ci) = abc[i];
            graph[ai].push((bi, ci));
            graph[bi].push((ai, ci));
            edges.insert((ai, bi), ci);
            edges.insert((bi, ai), ci);
            cost[i] = s;
        }
    }

    let mut parent = vec![Some(INF); n];
    parent[0] = None;
    let mut queue = VecDeque::new();
    for &(v, _) in &graph[0] {
        queue.push_back((v, 0));
    }
    while let Some((v, u)) = queue.pop_front() {
        parent[v] = Some(u);
        for &(w, _) in &graph[v] {
            if parent[w] == Some(INF) {
                queue.push_back((w, v));
            }
        }
    }
    // eprintln!("{:?}", parent);

    for i in 0..m {
        if cost[i] != INF {
            continue;
        }
        let (ai, bi, ci) = abc[i];
        let mut ancestors = HashSet::new();
        ancestors.insert(ai);
        let mut u = ai;
        while let Some(v) = parent[u] {
            ancestors.insert(v);
            u = v;
        }
        let w = if ancestors.contains(&bi) {
            bi
        } else {
            let mut w = INF;
            let mut u = bi;
            while let Some(v) = parent[u] {
                if ancestors.contains(&v) {
                    w = v;
                    break;
                }
                u = v;
            }
            w
        };

        // eprintln!("{}", w);
        let mut cycle = vec![];
        let mut u = ai;
        while let Some(v) = parent[u] {
            if u == w {
                break;
            }
            cycle.push((u, v));
            u = v;
        }
        let mut u = bi;
        while let Some(v) = parent[u] {
            if u == w {
                break;
            }
            cycle.push((u, v));
            u = v;
        }
        // eprintln!("{} {}", ai, bi);
        // eprintln!("{:?}", cycle);
        let di = cycle.iter().map(|&(x, y)| edges[&(x, y)]).max().unwrap();
        cost[i] = s - di + ci;
    }

    for i in 0..m {
        println!("{}", cost[i]);
    }
}
