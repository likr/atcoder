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
        mut tax: [(usize, Usize1, usize); m],
    }
    let mut row_index = (0..h).collect::<HashSet<_>>();
    let mut col_index = (0..w).collect::<HashSet<_>>();
    let mut count = HashMap::new();
    tax.reverse();
    for &(ti, ai, xi) in tax.iter() {
        if ti == 1 {
            if row_index.contains(&ai) {
                if xi != 0 && col_index.len() > 0 {
                    *count.entry(xi).or_insert(0) += col_index.len();
                }
                row_index.remove(&ai);
            }
        } else {
            if col_index.contains(&ai) {
                if xi != 0 && row_index.len() > 0 {
                    *count.entry(xi).or_insert(0) += row_index.len();
                }
                col_index.remove(&ai);
            }
        }
    }
    let mut s = h * w;
    let mut ans = vec![];
    for (&c, &count) in count.iter() {
        s -= count;
        ans.push((c, count));
    }
    if s > 0 {
        ans.push((0, s));
    }
    ans.sort();
    println!("{}", ans.len());
    for &(c, count) in ans.iter() {
        println!("{} {}", c, count);
    }
}
