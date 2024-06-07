use ac_library::*;
use itertools::*;
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
        mut hwd: [(usize, usize, usize); n],
    }
    let mut ws = vec![];
    for i in 0..n {
        let (hi, wi, di) = hwd[i];
        hwd.push((hi, di, wi));
        hwd.push((wi, hi, di));
        hwd.push((wi, di, hi));
        hwd.push((di, hi, wi));
        hwd.push((di, wi, hi));
        ws.push(hi);
        ws.push(wi);
        ws.push(di);
    }
    ws.sort();
    ws.dedup();
    let m = ws.len();
    let mut index = HashMap::new();
    for i in 0..m {
        index.insert(ws[i], i);
    }
    hwd.sort_by_key(|&(hi, wi, di)| (di, wi, hi));
    let mut segtree = Segtree::<Min<usize>>::new(m);

    for (_, items) in hwd.iter().group_by(|&&(_, _, d)| d).into_iter() {
        let items = items.collect::<Vec<_>>();
        for &&(h, w, _) in items.iter() {
            if segtree.prod(..index[&w]) < h {
                println!("Yes");
                return;
            }
        }
        for &&(h, w, _) in items.iter() {
            segtree.set(index[&w], std::cmp::min(segtree.get(index[&w]), h));
        }
    }
    println!("No");
}
