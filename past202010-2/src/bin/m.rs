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
        ab: [(Usize1, Usize1); n - 1],
        mut uvc: [(Usize1, Usize1, usize); q],
    }

    let root = 0;
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut height = vec![INF; n];
    height[root] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(root);
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if height[v] == INF {
                height[v] = height[u] + 1;
                queue.push_back(v);
            }
        }
    }
    debug!(height);
    let mut parent = vec![None; n];
    for &(ai, bi) in &ab {
        if height[ai] > height[bi] {
            parent[ai] = Some(bi);
        } else {
            parent[bi] = Some(ai);
        }
    }
    debug!(parent);

    let mut graph = vec![HashSet::new(); n];
    for &(ai, bi) in &ab {
        if height[ai] > height[bi] {
            graph[bi].insert(ai);
        } else {
            graph[ai].insert(bi);
        }
    }

    let mut color = HashMap::new();
    let edges = ab.iter().collect::<HashSet<_>>();
    let mut children = vec![];
    let mut visited = HashSet::new();
    uvc.reverse();
    for &(ui, vi, ci) in &uvc {
        children.clear();
        visited.clear();

        let mut x = ui;
        let mut y = vi;
        if height[x] > height[y] {
            std::mem::swap(&mut x, &mut y);
        }

        while height[y] > height[x] {
            visited.insert(y);
            let z = parent[y].unwrap();
            if edges.contains(&(y, z)) && !color.contains_key(&(y, z)) {
                debug!((y, z));
                color.insert((y, z), ci);
            }
            if edges.contains(&(z, y)) && !color.contains_key(&(z, y)) {
                debug!((z, y));
                color.insert((z, y), ci);
            }
            y = z;
        }
        while x != y {
            visited.insert(x);
            visited.insert(y);
            let z = parent[x].unwrap();
            if edges.contains(&(x, z)) && !color.contains_key(&(x, z)) {
                debug!((x, z));
                color.insert((x, z), ci);
            }
            if edges.contains(&(z, x)) && !color.contains_key(&(z, x)) {
                debug!((z, x));
                color.insert((z, x), ci);
            }
            x = z;
            let z = parent[y].unwrap();
            if edges.contains(&(y, z)) && !color.contains_key(&(y, z)) {
                debug!((y, z));
                color.insert((y, z), ci);
            }
            if edges.contains(&(z, y)) && !color.contains_key(&(z, y)) {
                debug!((z, y));
                color.insert((z, y), ci);
            }
            y = z;
        }
        debug!(visited);

        for &u in visited.iter() {
            for &v in &graph[u] {
                if !visited.contains(&v) {
                    children.push(v);
                }
            }
        }
        debug!(children);

        let w = x;
        debug!(w);

        for &v in &children {
            graph[w].insert(v);
            parent[v] = Some(w);
            height[v] = height[w] + 1;
        }
        for &u in visited.iter() {
            parent[u] = Some(w);
            graph[w].remove(&u);
            graph[u].clear();
        }
        debug!(graph);
        debug!(height);
        debug!(parent);
        debug!(color);
    }

    for &(ai, bi) in &ab {
        if let Some(c) = color.get(&(ai, bi)) {
            println!("{}", c);
        } else {
            println!("0");
        }
    }
}
