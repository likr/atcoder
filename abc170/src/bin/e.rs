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
        mut ab: [(usize, Usize1); n],
        cd: [(Usize1, Usize1); q],
    }
    let m = 200000;
    let mut school_infants = vec![BTreeSet::new(); m];
    let mut strongest_infants = BTreeSet::new();
    for i in 0..n {
        school_infants[ab[i].1].insert((ab[i].0, i));
    }
    for i in 0..m {
        if let Some(&s) = school_infants[i].range(..).rev().nth(0) {
            strongest_infants.insert(s);
        }
    }
    for i in 0..q {
        let (ci, di) = cd[i];
        strongest_infants.remove(&*school_infants[ab[ci].1].range(..).rev().nth(0).unwrap());
        school_infants[ab[ci].1].remove(&(ab[ci].0, ci));
        if let Some(&t) = school_infants[ab[ci].1].range(..).rev().nth(0) {
            strongest_infants.insert(t);
        }
        ab[ci].1 = di;
        if let Some(&t) = school_infants[ab[ci].1].range(..).rev().nth(0) {
            strongest_infants.remove(&t);
        }
        school_infants[ab[ci].1].insert((ab[ci].0, ci));
        strongest_infants.insert(*school_infants[ab[ci].1].range(..).rev().nth(0).unwrap());
        println!("{}", strongest_infants.range(..).nth(0).unwrap().0);
    }
}
