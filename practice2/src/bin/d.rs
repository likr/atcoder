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
        mut s: [Chars; n],
    }

    let index = |i, j| i * m + j;
    let mut graph = MfGraph::new(n * m + 2);
    let mut edges = HashMap::new();
    for i in 0..n {
        for j in 1..m {
            if (i + j) % 2 == 0 {
                edges.insert(
                    graph.add_edge(index(i, j), index(i, j - 1), 1),
                    (i, j - 1, i, j),
                );
            } else {
                edges.insert(
                    graph.add_edge(index(i, j - 1), index(i, j), 1),
                    (i, j - 1, i, j),
                );
            }
        }
    }
    for i in 1..n {
        for j in 0..m {
            if (i + j) % 2 == 0 {
                edges.insert(
                    graph.add_edge(index(i, j), index(i - 1, j), 1),
                    (i - 1, j, i, j),
                );
            } else {
                edges.insert(
                    graph.add_edge(index(i - 1, j), index(i, j), 1),
                    (i - 1, j, i, j),
                );
            }
        }
    }
    let source = n * m;
    let target = n * m + 1;
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == '.' {
                if (i + j) % 2 == 0 {
                    graph.add_edge(source, index(i, j), 1);
                } else {
                    graph.add_edge(index(i, j), target, 1);
                }
            }
        }
    }
    println!("{}", graph.flow(source, target));
    for (k, e) in graph.edges().iter().enumerate() {
        debug!(k, e);
        if let Some(&(i1, j1, i2, j2)) = edges.get(&k) {
            if e.flow == 1 {
                if i1 == i2 {
                    s[i1][j1] = '>';
                    s[i2][j2] = '<';
                } else {
                    s[i1][j1] = 'v';
                    s[i2][j2] = '^';
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", s[i].iter().collect::<String>());
    }
}
//https://github.com/rust-lang-ja/ac-library-rs

pub mod internal_queue {
    #![allow(dead_code)]

    #[derive(Default)]
    pub(crate) struct SimpleQueue<T> {
        payload: Vec<T>,
        pos: usize,
    }

    impl<T> SimpleQueue<T> {
        pub(crate) fn reserve(&mut self, n: usize) {
            if n > self.payload.len() {
                self.payload.reserve(n - self.payload.len());
            }
        }

        pub(crate) fn size(&self) -> usize {
            self.payload.len() - self.pos
        }

        pub(crate) fn empty(&self) -> bool {
            self.pos == self.payload.len()
        }

        pub(crate) fn push(&mut self, t: T) {
            self.payload.push(t);
        }

        // Do we need mutable version?
        pub(crate) fn front(&self) -> Option<&T> {
            if self.pos < self.payload.len() {
                Some(&self.payload[self.pos])
            } else {
                None
            }
        }

        pub(crate) fn clear(&mut self) {
            self.payload.clear();
            self.pos = 0;
        }

        pub(crate) fn pop(&mut self) -> Option<&T> {
            if self.pos < self.payload.len() {
                self.pos += 1;
                Some(&self.payload[self.pos - 1])
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod test {
        use crate::internal_queue::SimpleQueue;

        #[allow(clippy::cognitive_complexity)]
        #[test]
        fn test_simple_queue() {
            let mut queue = SimpleQueue::default();

            assert_eq!(queue.size(), 0);
            assert!(queue.empty());
            assert!(queue.front().is_none());
            assert!(queue.pop().is_none());

            queue.push(123);

            assert_eq!(queue.size(), 1);
            assert!(!queue.empty());
            assert_eq!(queue.front(), Some(&123));

            queue.push(456);

            assert_eq!(queue.size(), 2);
            assert!(!queue.empty());
            assert_eq!(queue.front(), Some(&123));

            assert_eq!(queue.pop(), Some(&123));
            assert_eq!(queue.size(), 1);
            assert!(!queue.empty());
            assert_eq!(queue.front(), Some(&456));

            queue.push(789);
            queue.push(789);
            queue.push(456);
            queue.push(456);

            assert_eq!(queue.size(), 5);
            assert!(!queue.empty());
            assert_eq!(queue.front(), Some(&456));

            assert_eq!(queue.pop(), Some(&456));
            assert_eq!(queue.size(), 4);
            assert!(!queue.empty());
            assert_eq!(queue.front(), Some(&789));

            queue.clear();

            assert_eq!(queue.size(), 0);
            assert!(queue.empty());
            assert!(queue.front().is_none());
            assert!(queue.pop().is_none());
        }
    }
}
pub mod internal_type_traits {
    use std::{
        fmt,
        iter::{Product, Sum},
        ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        },
    };

    // Skipped:
    //
    // - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
    // - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
    // - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

    /// Corresponds to `std::is_integral` in C++.
    // We will remove unnecessary bounds later.
    //
    // Maybe we should rename this to `PrimitiveInteger` or something, as it probably won't be used in the
    // same way as the original ACL.
    pub trait Integral:
        'static
        + Send
        + Sync
        + Copy
        + Ord
        + Not<Output = Self>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Sum
        + Product
        + BitOr<Output = Self>
        + BitAnd<Output = Self>
        + BitXor<Output = Self>
        + BitOrAssign
        + BitAndAssign
        + BitXorAssign
        + Shl<Output = Self>
        + Shr<Output = Self>
        + ShlAssign
        + ShrAssign
        + fmt::Display
        + fmt::Debug
        + fmt::Binary
        + fmt::Octal
        + Zero
        + One
        + BoundedBelow
        + BoundedAbove
    {
    }

    /// Class that has additive identity element
    pub trait Zero {
        /// The additive identity element
        fn zero() -> Self;
    }

    /// Class that has multiplicative identity element
    pub trait One {
        /// The multiplicative identity element
        fn one() -> Self;
    }

    pub trait BoundedBelow {
        fn min_value() -> Self;
    }

    pub trait BoundedAbove {
        fn max_value() -> Self;
    }

    macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }

            impl Integral for $ty {}
        )*
    };
}

    impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
}
pub mod maxflow {
    #![allow(dead_code)]
    use crate::internal_queue::SimpleQueue;
    use crate::internal_type_traits::Integral;
    use std::cmp::min;
    use std::iter;

    impl<Cap> MfGraph<Cap>
    where
        Cap: Integral,
    {
        pub fn new(n: usize) -> MfGraph<Cap> {
            MfGraph {
                _n: n,
                pos: Vec::new(),
                g: iter::repeat_with(Vec::new).take(n).collect(),
            }
        }

        pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) -> usize {
            assert!(from < self._n);
            assert!(to < self._n);
            assert!(Cap::zero() <= cap);
            let m = self.pos.len();
            self.pos.push((from, self.g[from].len()));
            let rev = self.g[to].len() + if from == to { 1 } else { 0 };
            self.g[from].push(_Edge { to, rev, cap });
            let rev = self.g[from].len() - 1;
            self.g[to].push(_Edge {
                to: from,
                rev,
                cap: Cap::zero(),
            });
            m
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Edge<Cap: Integral> {
        pub from: usize,
        pub to: usize,
        pub cap: Cap,
        pub flow: Cap,
    }

    impl<Cap> MfGraph<Cap>
    where
        Cap: Integral,
    {
        pub fn get_edge(&self, i: usize) -> Edge<Cap> {
            let m = self.pos.len();
            assert!(i < m);
            let _e = &self.g[self.pos[i].0][self.pos[i].1];
            let _re = &self.g[_e.to][_e.rev];
            Edge {
                from: self.pos[i].0,
                to: _e.to,
                cap: _e.cap + _re.cap,
                flow: _re.cap,
            }
        }
        pub fn edges(&self) -> Vec<Edge<Cap>> {
            let m = self.pos.len();
            (0..m).map(|i| self.get_edge(i)).collect()
        }
        pub fn change_edge(&mut self, i: usize, new_cap: Cap, new_flow: Cap) {
            let m = self.pos.len();
            assert!(i < m);
            assert!(Cap::zero() <= new_flow && new_flow <= new_cap);
            let (to, rev) = {
                let _e = &mut self.g[self.pos[i].0][self.pos[i].1];
                _e.cap = new_cap - new_flow;
                (_e.to, _e.rev)
            };
            let _re = &mut self.g[to][rev];
            _re.cap = new_flow;
        }

        /// `s != t` must hold, otherwise it panics.
        pub fn flow(&mut self, s: usize, t: usize) -> Cap {
            self.flow_with_capacity(s, t, Cap::max_value())
        }
        /// # Parameters
        /// * `s != t` must hold, otherwise it panics.
        /// * `flow_limit >= 0`
        pub fn flow_with_capacity(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {
            let n_ = self._n;
            assert!(s < n_);
            assert!(t < n_);
            // By the definition of max flow in appendix.html, this function should return 0
            // when the same vertices are provided.  On the other hand, it is reasonable to
            // return infinity-like value too, which is what the original implementation
            // (and this implementation without the following assertion) does.
            // Since either return value is confusing, we'd rather deny the parameters
            // of the two same vertices.
            // For more details, see https://github.com/rust-lang-ja/ac-library-rs/pull/24#discussion_r485343451
            // and https://github.com/atcoder/ac-library/issues/5 .
            assert_ne!(s, t);
            // Additional constraint
            assert!(Cap::zero() <= flow_limit);

            let mut calc = FlowCalculator {
                graph: self,
                s,
                t,
                flow_limit,
                level: vec![0; n_],
                iter: vec![0; n_],
                que: SimpleQueue::default(),
            };

            let mut flow = Cap::zero();
            while flow < flow_limit {
                calc.bfs();
                if calc.level[t] == -1 {
                    break;
                }
                calc.iter.iter_mut().for_each(|e| *e = 0);
                while flow < flow_limit {
                    let f = calc.dfs(t, flow_limit - flow);
                    if f == Cap::zero() {
                        break;
                    }
                    flow += f;
                }
            }
            flow
        }

        pub fn min_cut(&self, s: usize) -> Vec<bool> {
            let mut visited = vec![false; self._n];
            let mut que = SimpleQueue::default();
            que.push(s);
            while !que.empty() {
                let &p = que.front().unwrap();
                que.pop();
                visited[p] = true;
                for e in &self.g[p] {
                    if e.cap != Cap::zero() && !visited[e.to] {
                        visited[e.to] = true;
                        que.push(e.to);
                    }
                }
            }
            visited
        }
    }

    struct FlowCalculator<'a, Cap> {
        graph: &'a mut MfGraph<Cap>,
        s: usize,
        t: usize,
        flow_limit: Cap,
        level: Vec<i32>,
        iter: Vec<usize>,
        que: SimpleQueue<usize>,
    }

    impl<Cap> FlowCalculator<'_, Cap>
    where
        Cap: Integral,
    {
        fn bfs(&mut self) {
            self.level.iter_mut().for_each(|e| *e = -1);
            self.level[self.s] = 0;
            self.que.clear();
            self.que.push(self.s);
            while !self.que.empty() {
                let v = *self.que.front().unwrap();
                self.que.pop();
                for e in &self.graph.g[v] {
                    if e.cap == Cap::zero() || self.level[e.to] >= 0 {
                        continue;
                    }
                    self.level[e.to] = self.level[v] + 1;
                    if e.to == self.t {
                        return;
                    }
                    self.que.push(e.to);
                }
            }
        }
        fn dfs(&mut self, v: usize, up: Cap) -> Cap {
            if v == self.s {
                return up;
            }
            let mut res = Cap::zero();
            let level_v = self.level[v];
            for i in self.iter[v]..self.graph.g[v].len() {
                self.iter[v] = i;
                let &_Edge {
                    to: e_to,
                    rev: e_rev,
                    ..
                } = &self.graph.g[v][i];
                if level_v <= self.level[e_to] || self.graph.g[e_to][e_rev].cap == Cap::zero() {
                    continue;
                }
                let d = self.dfs(e_to, min(up - res, self.graph.g[e_to][e_rev].cap));
                if d <= Cap::zero() {
                    continue;
                }
                self.graph.g[v][i].cap += d;
                self.graph.g[e_to][e_rev].cap -= d;
                res += d;
                if res == up {
                    break;
                }
            }
            self.iter[v] = self.graph.g[v].len();
            res
        }
    }

    #[derive(Default)]
    pub struct MfGraph<Cap> {
        _n: usize,
        pos: Vec<(usize, usize)>,
        g: Vec<Vec<_Edge<Cap>>>,
    }

    struct _Edge<Cap> {
        to: usize,
        rev: usize,
        cap: Cap,
    }

    #[cfg(test)]
    mod test {
        use crate::{Edge, MfGraph};

        #[test]
        fn test_max_flow_wikipedia() {
            // From https://commons.wikimedia.org/wiki/File:Min_cut.png
            // Under CC BY-SA 3.0 https://creativecommons.org/licenses/by-sa/3.0/deed.en
            let mut graph = MfGraph::new(6);
            assert_eq!(graph.add_edge(0, 1, 3), 0);
            assert_eq!(graph.add_edge(0, 2, 3), 1);
            assert_eq!(graph.add_edge(1, 2, 2), 2);
            assert_eq!(graph.add_edge(1, 3, 3), 3);
            assert_eq!(graph.add_edge(2, 4, 2), 4);
            assert_eq!(graph.add_edge(3, 4, 4), 5);
            assert_eq!(graph.add_edge(3, 5, 2), 6);
            assert_eq!(graph.add_edge(4, 5, 3), 7);

            assert_eq!(graph.flow(0, 5), 5);

            let edges = graph.edges();
            {
                #[rustfmt::skip]
            assert_eq!(
                edges,
                vec![
                    Edge { from: 0, to: 1, cap: 3, flow: 3 },
                    Edge { from: 0, to: 2, cap: 3, flow: 2 },
                    Edge { from: 1, to: 2, cap: 2, flow: 0 },
                    Edge { from: 1, to: 3, cap: 3, flow: 3 },
                    Edge { from: 2, to: 4, cap: 2, flow: 2 },
                    Edge { from: 3, to: 4, cap: 4, flow: 1 },
                    Edge { from: 3, to: 5, cap: 2, flow: 2 },
                    Edge { from: 4, to: 5, cap: 3, flow: 3 },
                ]
            );
            }
            assert_eq!(
                graph.min_cut(0),
                vec![true, false, true, false, false, false]
            );
        }

        #[test]
        fn test_max_flow_wikipedia_multiple_edges() {
            // From https://commons.wikimedia.org/wiki/File:Min_cut.png
            // Under CC BY-SA 3.0 https://creativecommons.org/licenses/by-sa/3.0/deed.en
            let mut graph = MfGraph::new(6);
            for &(u, v, c) in &[
                (0, 1, 3),
                (0, 2, 3),
                (1, 2, 2),
                (1, 3, 3),
                (2, 4, 2),
                (3, 4, 4),
                (3, 5, 2),
                (4, 5, 3),
            ] {
                for _ in 0..c {
                    graph.add_edge(u, v, 1);
                }
            }

            assert_eq!(graph.flow(0, 5), 5);
            assert_eq!(
                graph.min_cut(0),
                vec![true, false, true, false, false, false]
            );
        }

        #[test]
        #[allow(clippy::many_single_char_names)]
        fn test_max_flow_misawa() {
            // Originally by @MiSawa
            // From https://gist.github.com/MiSawa/47b1d99c372daffb6891662db1a2b686
            let n = 100;

            let mut graph = MfGraph::new((n + 1) * 2 + 5);
            let (s, a, b, c, t) = (0, 1, 2, 3, 4);
            graph.add_edge(s, a, 1);
            graph.add_edge(s, b, 2);
            graph.add_edge(b, a, 2);
            graph.add_edge(c, t, 2);
            for i in 0..n {
                let i = 2 * i + 5;
                for j in 0..2 {
                    for k in 2..4 {
                        graph.add_edge(i + j, i + k, 3);
                    }
                }
            }
            for j in 0..2 {
                graph.add_edge(a, 5 + j, 3);
                graph.add_edge(2 * n + 5 + j, c, 3);
            }

            assert_eq!(graph.flow(s, t), 2);
        }
    }
}
use maxflow::*;
