use proconio::input;
use std::collections::HashSet;

const M: usize = 1000000007;

fn count_children(
    graph: &Vec<Vec<usize>>,
    u: usize,
    nodes: &mut Vec<usize>,
    leaves: &mut Vec<usize>,
    visited: &mut HashSet<usize>,
) {
    visited.insert(u);
    nodes[u] = 1;
    leaves[u] = if graph[u].len() == 1 { 1 } else { 0 };
    for &v in &graph[u] {
        if !visited.contains(&v) {
            count_children(graph, v, nodes, leaves, visited);
            nodes[u] += nodes[v];
            leaves[u] += leaves[v];
        }
    }
}

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn main() {
    input! {
      n: usize,
      ab: [(usize, usize); n - 1],
    }
    let mut graph = vec![vec![]; n + 1];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut root = 0;
    for u in 1..=n {
        if graph[u].len() > graph[root].len() {
            root = u;
        }
    }

    let mut nodes = vec![0; n + 1];
    let mut leaves = vec![0; n + 1];
    let mut visited = HashSet::new();
    count_children(&graph, root, &mut nodes, &mut leaves, &mut visited);
    eprintln!("{:?}", nodes);
    eprintln!("{:?}", leaves);

    let mut f = vec![0; n + 1];
    f[0] = 1;
    for i in 1..=n {
        f[i] = f[i - 1] * (i % M) % M;
    }

    let mut x = 1;
    for _ in 0..n {
        x = x * 2 % M;
    }
    let mut y = 0;
    for u in 1..=n {
        let nu = nodes[u];
        let lu = leaves[u];
        let mu = nu - lu;
        if lu == 1 {
            continue;
        }
        let mut a = 0;
        for k in 2..=lu {
            let num = f[lu];
            let denom = f[k] * f[lu - k] % M;
            a = (a + num * inv(denom) % M) % M;
        }
        let mut b = 0;
        for k in 0..=mu {
            let num = f[mu];
            let denom = f[k] * f[mu - k] % M;
            b = (b + k * (num * inv(denom) % M) % M) % M;
        }
        eprintln!("{} {}", a, b);
        y = (y + a * b % M) % M;
    }
    println!("{}", y * inv(x) % M);
}
