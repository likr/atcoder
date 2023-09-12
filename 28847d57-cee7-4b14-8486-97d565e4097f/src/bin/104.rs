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
        c: [Usize1; n],
    }
    let mut dsu = Dsu::new(n);
    let mut size = vec![HashMap::new(); n];
    for i in 0..n {
        size[i].insert(c[i], 1);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                a: Usize1,
                b: Usize1,
            }
            if !dsu.same(a, b) {
                let (u, v) = if dsu.size(a) < dsu.size(b) {
                    (dsu.leader(a), dsu.leader(b))
                } else {
                    (dsu.leader(b), dsu.leader(a))
                };
                for (&k, &c) in size[u].clone().iter() {
                    *size[v].entry(k).or_insert(0) += c;
                }
                dsu.merge(a, b);
            }
        } else {
            input! {
                x: Usize1,
                y: Usize1,
            }
            if let Some(&ans) = size[dsu.leader(x)].get(&y) {
                println!("{}", ans);
            } else {
                println!("0");
            }
        }
    }
}
