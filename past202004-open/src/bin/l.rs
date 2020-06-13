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
macro_rules! mid {
    ($b:expr, $e:expr) => {
        $b + ($e - $b) / 2
    };
}

/// For the internal nodes to be able to have
/// aggregations of their children, the type of the node
/// must define how to aggregate with other instance
pub trait Aggregator: Default + Clone {
    /// Aggregation function
    fn aggregate(&self, other: Self) -> Self;
}

///
pub struct SegmentTree<T> {
    tree: Vec<T>,
    n: usize,
}

impl<T> SegmentTree<T>
where
    T: Aggregator,
{
    fn build_helper<U>(st: &mut SegmentTree<T>, source: &[U], n: usize, b: usize, e: usize)
    where
        U: Into<T> + Clone,
    {
        if b > e {
            return;
        }
        if b == e {
            st.tree[n] = source[b].clone().into();
            return;
        }
        // left child
        SegmentTree::build_helper(st, source, 2 * n, b, mid!(b, e));
        // right child
        SegmentTree::build_helper(st, source, 2 * n + 1, mid!(b, e) + 1, e);
        // done
        st.tree[n] = st.tree[2 * n].aggregate(st.tree[2 * n + 1].clone());
    }

    ///
    pub fn build<U>(source: &[U]) -> SegmentTree<T>
    where
        U: Into<T> + Clone,
    {
        let mut st = SegmentTree {
            tree: vec![
                Default::default();
                {
                    let tree_height = (source.len() as f64).log2().ceil() as u32;
                    2 * (2 as usize).pow(tree_height) // maximum balanced tree size
                }
            ],
            n: source.len(),
        };
        SegmentTree::build_helper(&mut st, source, 1, 0, source.len() - 1);
        st
    }
}

impl<T> SegmentTree<T>
where
    T: Aggregator,
{
    fn query_helper(&self, n: usize, b: usize, e: usize, i: usize, j: usize) -> T {
        if e < i || b > j {
            return Default::default();
        }
        if b >= i && e <= j {
            return self.tree[n].clone();
        }
        self.query_helper(n * 2, b, mid!(b, e), i, j)
            .aggregate(self.query_helper(n * 2 + 1, mid!(b, e) + 1, e, i, j))
    }

    pub fn query(&self, i: usize, j: usize) -> T {
        self.query_helper(1, 0, self.n - 1, i, j)
    }
}

impl<T> SegmentTree<T>
where
    T: Aggregator,
{
    fn update_helper<U>(&mut self, n: usize, b: usize, e: usize, i: usize, v: &U)
    where
        U: Into<T> + Clone,
    {
        if b > e || b > i || e < i {
            return;
        }
        if b == e {
            self.tree[n] = v.clone().into();
            return;
        }
        self.update_helper(n * 2, b, mid!(b, e), i, v);
        self.update_helper(n * 2 + 1, mid!(b, e) + 1, e, i, v);
        self.tree[n] = self.tree[n * 2].aggregate(self.tree[n * 2 + 1].clone());
    }

    pub fn update<U>(&mut self, i: usize, v: U)
    where
        U: Into<T> + Clone,
    {
        let m = self.n - 1;
        self.update_helper(1, 0, m, i, &v);
    }
}

impl<T> SegmentTree<T>
where
    T: Clone,
{
    fn make_vec_helper<U>(&self, res: &mut Vec<U>, n: usize, b: usize, e: usize)
    where
        T: Into<U>,
    {
        if b > e {
            return;
        }
        if b == e {
            res.push(self.tree[n].clone().into());
            return;
        }
        self.make_vec_helper(res, 2 * n, b, mid!(b, e));
        self.make_vec_helper(res, 2 * n + 1, mid!(b, e) + 1, e);
    }

    pub fn make_vec<U>(&self) -> Vec<U>
    where
        T: Into<U>,
    {
        let mut res = vec![];
        self.make_vec_helper(&mut res, 1, 0, self.n - 1);
        res
    }
}

#[derive(Debug, Clone)]
struct Min((usize, usize));

impl Aggregator for Min {
    fn aggregate(&self, other: Min) -> Min {
        Min(min(self.0, other.0))
    }
}

impl Default for Min {
    fn default() -> Min {
        Min((INF, INF))
    }
}

// impl From<(usize, usize)> for Min {
//     fn from(i: (usize, usize)) -> Min {
//         Min(i)
//     }
// }
//
// impl Into<(usize, usize)> for Min {
//     fn into(self) -> (usize, usize) {
//         self.0
//     }
// }

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    }
    let a = (0..n).map(|i| Min((a[i], i))).collect::<Vec<_>>();
    let st = SegmentTree::<Min>::build(&a);
    let mut result = vec![];
    let mut left = 0;
    for i in 0..k {
        if k * d > n + (i + 1) * d {
            println!("-1");
            return;
        }
        let right = n - (k - i - 1) * d;
        if right <= left {
            println!("-1");
            return;
        }
        // eprintln!("{} {}", left, right);
        let Min((aj, j)) = st.query(left, right - 1);
        // eprintln!(" {} {}", aj, j);
        result.push(aj);
        left = j + d;
    }
    for i in 0..k {
        println!("{}", result[i]);
    }
}
