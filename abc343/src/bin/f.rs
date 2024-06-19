use ac_library::*;
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
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[derive(Clone)]
struct M {
    first: i64,
    second: i64,
    first_count: i64,
    second_count: i64,
}

impl Monoid for M {
    type S = M;

    fn identity() -> Self::S {
        M {
            first: -INF + 1,
            second: -INF,
            first_count: 0,
            second_count: 0,
        }
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        let mut values = HashMap::new();
        *values.entry(a.first).or_insert(0) += a.first_count;
        *values.entry(a.second).or_insert(0) += a.second_count;
        *values.entry(b.first).or_insert(0) += b.first_count;
        *values.entry(b.second).or_insert(0) += b.second_count;
        let mut values = values.into_iter().collect::<Vec<_>>();
        values.sort();
        values.reverse();
        M {
            first: values[0].0,
            first_count: values[0].1,
            second: values[1].0,
            second_count: values[1].1,
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut tree = Segtree::<M>::new(n);
    for i in 0..n {
        tree.set(
            i,
            M {
                first: a[i],
                first_count: 1,
                second: -INF,
                second_count: 0,
            },
        )
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                p: Usize1,
                x: i64,
            }
            tree.set(
                p,
                M {
                    first: x,
                    first_count: 1,
                    second: -INF,
                    second_count: 0,
                },
            )
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            println!("{}", tree.prod(l..=r).second_count);
        }
    }
}
