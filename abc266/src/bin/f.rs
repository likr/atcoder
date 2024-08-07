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

fn dfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    parent: &mut Vec<Option<usize>>,
    cycle: &mut HashSet<(usize, usize)>,
) {
    for &v in graph[u].iter() {
        if parent[v] == Some(INF) {
            parent[v] = Some(u);
            dfs(graph, v, parent, cycle);
        } else {
            if cycle.len() == 0 && parent[u].is_some() && parent[u].unwrap() != v {
                debug!(u, v);
                debug!(parent);
                cycle.insert((u, v));
                cycle.insert((v, u));
                let mut w = u;
                loop {
                    debug!(w, parent[w].unwrap());
                    cycle.insert((w, parent[w].unwrap()));
                    cycle.insert((parent[w].unwrap(), w));
                    w = parent[w].unwrap();
                    if w == v {
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n {
        let (ui, vi) = uv[i];
        graph[ui].push(vi);
        graph[vi].push(ui);
    }

    let mut parent = vec![Some(INF); n];
    parent[0] = None;
    let mut cycle = HashSet::new();
    dfs(&graph, 0, &mut parent, &mut cycle);

    let mut dsu = Dsu::new(n);
    for i in 0..n {
        let (ui, vi) = uv[i];
        if !cycle.contains(&(ui, vi)) {
            dsu.merge(ui, vi);
        }
    }

    for i in 0..q {
        let (xi, yi) = xy[i];
        if dsu.same(xi, yi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    //! A Disjoint set union (DSU) with union by size and path compression.

    /// A Disjoint set union (DSU) with union by size and path compression.
    ///
    /// See: [Zvi Galil and Giuseppe F. Italiano, Data structures and algorithms for disjoint set union problems](https://core.ac.uk/download/pdf/161439519.pdf)
    ///
    /// In the following documentation, let $n$ be the size of the DSU.
    ///
    /// # Example
    ///
    /// ```
    /// use ac_library_rs::Dsu;
    /// use proconio::{input, source::once::OnceSource};
    ///
    /// input! {
    ///     from OnceSource::from(
    ///         "5\n\
    ///          3\n\
    ///          0 1\n\
    ///          2 3\n\
    ///          3 4\n",
    ///     ),
    ///     n: usize,
    ///     abs: [(usize, usize)],
    /// }
    ///
    /// let mut dsu = Dsu::new(n);
    /// for (a, b) in abs {
    ///     dsu.merge(a, b);
    /// }
    ///
    /// assert!(dsu.same(0, 1));
    /// assert!(!dsu.same(1, 2));
    /// assert!(dsu.same(2, 4));
    ///
    /// assert_eq!(
    ///     dsu.groups()
    ///         .into_iter()
    ///         .map(|mut group| {
    ///             group.sort_unstable();
    ///             group
    ///         })
    ///         .collect::<Vec<_>>(),
    ///     [&[0, 1][..], &[2, 3, 4][..]],
    /// );
    /// ```
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        /// Creates a new `Dsu`.
        ///
        /// # Constraints
        ///
        /// - $0 \leq n \leq 10^8$
        ///
        /// # Complexity
        ///
        /// - $O(n)$
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }

        // `\textsc` does not work in KaTeX
        /// Performs the Uɴɪᴏɴ operation.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        /// - $0 \leq b < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraints are not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        /// Returns whether the vertices $a$ and $b$ are in the same connected component.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        /// - $0 \leq b < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraint is not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }

        /// Performs the Fɪɴᴅ operation.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraint is not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }

        /// Returns the size of the connected component that contains the vertex $a$.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraint is not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }

        /// Divides the graph into connected components.
        ///
        /// The result may not be ordered.
        ///
        /// # Complexity
        ///
        /// - $O(n)$
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert!(d.same(0, 1));
            d.merge(1, 2);
            assert!(d.same(0, 2));
            assert_eq!(d.size(0), 3);
            assert!(!d.same(0, 3));
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
use dsu::*;
