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
        ab: [(usize, Usize1); n],
        cd: [(Usize1, Usize1); q],
    }
    let m = 200000;
    let mut p = vec![BTreeSet::new(); m];
    let mut index = vec![0; n];
    for i in 0..n {
        let (ai, bi) = ab[i];
        p[bi].insert((ai, i));
        index[i] = bi;
    }
    let mut q = BTreeSet::new();
    for i in 0..m {
        if let Some(&x) = p[i].last() {
            q.insert(x);
        }
    }
    for &(k, j2) in cd.iter() {
        let ak = ab[k].0;
        let j1 = index[k];
        q.remove(p[j1].last().unwrap());
        p[j1].remove(&(ak, k));
        if let Some(&x) = p[j1].last() {
            q.insert(x);
        }
        if let Some(&x) = p[j2].last() {
            q.remove(&x);
        }
        p[j2].insert((ak, k));
        q.insert(*p[j2].last().unwrap());
        index[k] = j2;
        println!("{}", q.first().unwrap().0);
    }
}
