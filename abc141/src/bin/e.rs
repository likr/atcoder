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

struct RollingHash {
    bm: Vec<(usize, usize)>,
    hash: Vec<Vec<usize>>,
    pow: Vec<Vec<usize>>,
}

impl RollingHash {
    fn new(s: &[usize], bm: &[(usize, usize)]) -> RollingHash {
        let bm = bm.to_vec();
        let n = s.len();
        let m = bm.len();
        let mut pow = vec![vec![1; m]; n + 1];
        for i in 1..=n {
            for j in 0..m {
                let (bj, mj) = bm[j];
                pow[i][j] = pow[i - 1][j] * bj % mj;
            }
        }

        let mut hash = vec![vec![0; m]; n + 1];
        for i in 1..=n {
            for j in 0..m {
                let (bj, mj) = bm[j];
                hash[i][j] = (hash[i - 1][j] + s[i - 1]) * bj % mj;
            }
        }
        RollingHash { bm, hash, pow }
    }

    fn hash(&self, start: usize, stop: usize) -> Vec<usize> {
        (0..self.bm.len())
            .map(|j| {
                let (_, mj) = self.bm[j];
                (self.hash[stop][j] + mj - self.hash[start][j] * self.pow[stop - start][j] % mj)
                    % mj
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let s = s.into_iter().map(|c| c as usize).collect::<Vec<_>>();
    let rh = RollingHash::new(&s, &vec![(1009, 1000000007), (1007, 1000000009)]);

    let mut result = 0;
    let mut l_hash = HashMap::new();
    for l in 1..=n / 2 {
        l_hash.clear();
        for s in 0..=n - l {
            let h = rh.hash(s, s + l);
            l_hash.entry(h).or_insert(vec![]).push(s);
        }
        for indices in l_hash.values() {
            if indices[indices.len() - 1] - indices[0] >= l {
                result = l;
            }
        }
        debug!(l, l_hash);
    }
    println!("{}", result);
}
