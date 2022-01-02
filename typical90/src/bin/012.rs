use petgraph::unionfind::UnionFind;
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
        h: usize,
        w: usize,
        q: usize,
    }
    let mut uf = UnionFind::new(h * w);
    let mut visited = HashSet::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                r1: Usize1,
                c1: Usize1,
            }
            visited.insert((r1, c1));
            for &(dr, dc) in [(0, 1), (2, 1), (1, 0), (1, 2)].iter() {
                let r2 = r1 + dr;
                let c2 = c1 + dc;
                if 1 <= r2 && r2 <= h && 1 <= c2 && c2 <= w {
                    let r2 = r2 - 1;
                    let c2 = c2 - 1;
                    if visited.contains(&(r2, c2)) {
                        uf.union(r1 * w + c1, r2 * w + c2);
                    }
                }
            }
        } else {
            input! {
                r1: Usize1,
                c1: Usize1,
                r2: Usize1,
                c2: Usize1,
            }
            if visited.contains(&(r1, c1))
                && visited.contains(&(r2, c2))
                && uf.equiv(r1 * w + c1, r2 * w + c2)
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
