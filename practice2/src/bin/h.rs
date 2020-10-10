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
        d: i64,
        xy: [(i64, i64); n],
    }
    let mut sat = TwoSat::new(n);
    for i in 0..n {
        let (xi, yi) = xy[i];
        for j in 0..i {
            let (xj, yj) = xy[j];
            if (xi - xj).abs() < d {
                sat.add_clause(i, false, j, false);
            }
            if (xi - yj).abs() < d {
                sat.add_clause(i, false, j, true);
            }
            if (yi - xj).abs() < d {
                sat.add_clause(i, true, j, false);
            }
            if (yi - yj).abs() < d {
                sat.add_clause(i, true, j, true);
            }
        }
    }
    if sat.satisfiable() {
        println!("Yes");
        let sol = sat.answer();
        for i in 0..n {
            let (xi, yi) = xy[i];
            if sol[i] {
                println!("{}", xi);
            } else {
                println!("{}", yi);
            }
        }
    } else {
        println!("No");
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
pub mod twosat {
    use crate::internal_scc;

    pub struct TwoSat {
        n: usize,
        scc: internal_scc::SccGraph,
        answer: Vec<bool>,
    }
    impl TwoSat {
        pub fn new(n: usize) -> Self {
            TwoSat {
                n,
                answer: vec![false; n],
                scc: internal_scc::SccGraph::new(2 * n),
            }
        }
        pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
            assert!(i < self.n && j < self.n);
            self.scc.add_edge(2 * i + !f as usize, 2 * j + g as usize);
            self.scc.add_edge(2 * j + !g as usize, 2 * i + f as usize);
        }
        pub fn satisfiable(&mut self) -> bool {
            let id = self.scc.scc_ids().1;
            for i in 0..self.n {
                if id[2 * i] == id[2 * i + 1] {
                    return false;
                }
                self.answer[i] = id[2 * i] < id[2 * i + 1];
            }
            true
        }
        pub fn answer(&self) -> &[bool] {
            &self.answer
        }
    }

    #[cfg(test)]
    mod tests {
        #![allow(clippy::many_single_char_names)]
        use super::*;
        #[test]
        fn solve_alpc_h_sample1() {
            // https://atcoder.jp/contests/practice2/tasks/practice2_h

            let (n, d) = (3, 2);
            let x = [1, 2, 0i32];
            let y = [4, 5, 6];

            let mut t = TwoSat::new(n);

            for i in 0..n {
                for j in i + 1..n {
                    if (x[i] - x[j]).abs() < d {
                        t.add_clause(i, false, j, false);
                    }
                    if (x[i] - y[j]).abs() < d {
                        t.add_clause(i, false, j, true);
                    }
                    if (y[i] - x[j]).abs() < d {
                        t.add_clause(i, true, j, false);
                    }
                    if (y[i] - y[j]).abs() < d {
                        t.add_clause(i, true, j, true);
                    }
                }
            }
            assert!(t.satisfiable());
            let answer = t.answer();
            let mut res = vec![];
            for (i, &v) in answer.iter().enumerate() {
                if v {
                    res.push(x[i])
                } else {
                    res.push(y[i]);
                }
            }

            //Check the min distance between flags
            res.sort();
            let mut min_distance = i32::max_value();
            for i in 1..res.len() {
                min_distance = std::cmp::min(min_distance, res[i] - res[i - 1]);
            }
            assert!(min_distance >= d);
        }

        #[test]
        fn solve_alpc_h_sample2() {
            // https://atcoder.jp/contests/practice2/tasks/practice2_h

            let (n, d) = (3, 3);
            let x = [1, 2, 0i32];
            let y = [4, 5, 6];

            let mut t = TwoSat::new(n);

            for i in 0..n {
                for j in i + 1..n {
                    if (x[i] - x[j]).abs() < d {
                        t.add_clause(i, false, j, false);
                    }
                    if (x[i] - y[j]).abs() < d {
                        t.add_clause(i, false, j, true);
                    }
                    if (y[i] - x[j]).abs() < d {
                        t.add_clause(i, true, j, false);
                    }
                    if (y[i] - y[j]).abs() < d {
                        t.add_clause(i, true, j, true);
                    }
                }
            }
            assert!(!t.satisfiable());
        }
    }
}
use twosat::*;
