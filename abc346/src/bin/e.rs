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
        h: usize,
        w: usize,
        m: usize,
        tax: [(usize, Usize1, usize); m],
    }
    let mut rows = Segtree::<Additive<usize>>::new(h);
    let mut cols = Segtree::<Additive<usize>>::new(w);
    let mut ans = HashMap::new();
    for &(ti, ai, xi) in tax.iter().rev() {
        if ti == 1 {
            if rows.get(ai) == 0 {
                *ans.entry(xi).or_insert(0) += w - cols.all_prod();
                rows.set(ai, 1);
            }
        } else {
            if cols.get(ai) == 0 {
                *ans.entry(xi).or_insert(0) += h - rows.all_prod();
                cols.set(ai, 1);
            }
        }
    }
    let s = ans.values().sum::<usize>();
    *ans.entry(0).or_insert(0) += h * w - s;
    let mut ans = ans
        .into_iter()
        .filter(|&(_, count)| count > 0)
        .collect::<Vec<_>>();
    ans.sort();
    println!("{}", ans.len());
    for &(x, count) in ans.iter() {
        println!("{} {}", x, count);
    }
}
