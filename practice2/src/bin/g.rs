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
        ab: [(usize, usize); m],
    }
    let mut graph = SccGraph::new(n);
    for &(ai, bi) in &ab {
        graph.add_edge(ai, bi);
    }
    let scc = graph.scc();
    println!("{}", scc.len());
    for row in scc.iter() {
        println!(
            "{} {}",
            row.len(),
            row.iter()
                .map(|u| format!("{}", u))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod internal_scc {
    pub struct Csr<E> {
        start: Vec<usize>,
        elist: Vec<E>,
    }

    impl<E> Csr<E>
    where
        E: Copy,
    {
        pub fn new(n: usize, edges: &[(usize, E)], init: E) -> Self {
            let mut csr = Csr {
                start: vec![0; n + 1],
                elist: vec![init; edges.len()],
            };
            for e in edges.iter() {
                csr.start[e.0 + 1] += 1;
            }
            for i in 1..=n {
                csr.start[i] += csr.start[i - 1];
            }
            let mut counter = csr.start.clone();
            for e in edges.iter() {
                csr.elist[counter[e.0]] = e.1;
                counter[e.0] += 1;
            }
            csr
        }
    }

    #[derive(Copy, Clone)]
    struct _Edge {
        to: usize,
    }

    /// Reference:
    /// R. Tarjan,
    /// Depth-First Search and Linear Graph Algorithms
    pub struct SccGraph {
        n: usize,
        edges: Vec<(usize, _Edge)>,
    }

    impl SccGraph {
        pub fn new(n: usize) -> Self {
            SccGraph { n, edges: vec![] }
        }

        pub fn num_vertices(&self) -> usize {
            self.n
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            self.edges.push((from, _Edge { to }));
        }

        /// return pair of (# of scc, scc id)
        pub fn scc_ids(&self) -> (usize, Vec<usize>) {
            // In C++ ac-library, this function is implemented by using recursive lambda functions.
            // Instead, we use fn and struct for capturing environments.
            struct _Env {
                g: Csr<_Edge>,
                now_ord: usize,
                group_num: usize,
                visited: Vec<usize>,
                low: Vec<usize>,
                ord: Vec<Option<usize>>,
                ids: Vec<usize>,
            }
            let mut env = _Env {
                g: Csr::new(self.n, &self.edges, _Edge { to: 0 }),
                now_ord: 0,
                group_num: 0,
                visited: Vec::with_capacity(self.n),
                low: vec![0; self.n],
                ord: vec![None; self.n],
                ids: vec![0; self.n],
            };

            fn dfs(v: usize, n: usize, env: &mut _Env) {
                env.low[v] = env.now_ord;
                env.ord[v] = Some(env.now_ord);
                env.now_ord += 1;
                env.visited.push(v);

                for i in env.g.start[v]..env.g.start[v + 1] {
                    let to = env.g.elist[i].to;
                    if let Some(x) = env.ord[to] {
                        env.low[v] = std::cmp::min(env.low[v], x);
                    } else {
                        dfs(to, n, env);
                        env.low[v] = std::cmp::min(env.low[v], env.low[to]);
                    }
                }
                if env.low[v] == env.ord[v].unwrap() {
                    loop {
                        let u = *env.visited.last().unwrap();
                        env.visited.pop();
                        env.ord[u] = Some(n);
                        env.ids[u] = env.group_num;
                        if u == v {
                            break;
                        }
                    }
                    env.group_num += 1;
                }
            }
            for i in 0..self.n {
                if env.ord[i].is_none() {
                    dfs(i, self.n, &mut env);
                }
            }
            for x in env.ids.iter_mut() {
                *x = env.group_num - 1 - *x;
            }
            (env.group_num, env.ids)
        }

        pub fn scc(&self) -> Vec<Vec<usize>> {
            let ids = self.scc_ids();
            let group_num = ids.0;
            let mut counts = vec![0usize; group_num];
            for &x in ids.1.iter() {
                counts[x] += 1;
            }
            let mut groups: Vec<Vec<usize>> = (0..ids.0).map(|_| vec![]).collect();
            for i in 0..group_num {
                groups[i].reserve(counts[i]);
            }
            for i in 0..self.n {
                groups[ids.1[i]].push(i);
            }
            groups
        }
    }
}
pub mod scc {
    use crate::internal_scc;

    pub struct SccGraph {
        internal: internal_scc::SccGraph,
    }

    impl SccGraph {
        pub fn new(n: usize) -> Self {
            SccGraph {
                internal: internal_scc::SccGraph::new(n),
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize) {
            let n = self.internal.num_vertices();
            assert!(from < n);
            assert!(to < n);
            self.internal.add_edge(from, to);
        }

        pub fn scc(&self) -> Vec<Vec<usize>> {
            self.internal.scc()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_scc_simple() {
            let mut graph = SccGraph::new(2);
            graph.add_edge(0, 1);
            graph.add_edge(1, 0);
            let scc = graph.scc();
            assert_eq!(scc.len(), 1);
        }

        #[test]
        fn test_scc_self_loop() {
            let mut graph = SccGraph::new(2);
            graph.add_edge(0, 0);
            graph.add_edge(0, 0);
            graph.add_edge(1, 1);
            let scc = graph.scc();
            assert_eq!(scc.len(), 2);
        }

        #[test]
        fn solve_alpc_g_sample1() {
            // https://atcoder.jp/contests/practice2/tasks/practice2_g
            let n: usize = 6;
            let edges = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];

            let mut graph = SccGraph::new(n);
            for (u, v) in edges.into_iter() {
                graph.add_edge(u, v);
            }

            let scc = graph.scc();
            assert_eq!(scc, vec![vec![5], vec![1, 4], vec![2], vec![0, 3]]);
        }
    }
}
use scc::*;
