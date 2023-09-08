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
        pe: [[(usize, usize)]; n],
    }
    let mut ps = vec![];
    for i in 0..n {
        for &(pij, _) in pe[i].iter() {
            ps.push(pij);
        }
    }
    ps.sort();
    ps.dedup();
    let p_index = ps
        .iter()
        .enumerate()
        .map(|(k, pk)| (pk, k))
        .collect::<HashMap<_, _>>();
    let mut e_max = vec![0; ps.len()];
    for i in 0..n {
        for &(pij, eij) in pe[i].iter() {
            e_max[p_index[&pij]] = max(e_max[p_index[&pij]], eij);
        }
    }
    let mut max_count = vec![0usize; ps.len()];
    for i in 0..n {
        for &(pij, eij) in pe[i].iter() {
            if eij == e_max[p_index[&pij]] {
                max_count[p_index[&pij]] += 1;
            }
        }
    }
    let mut result = 0;
    for i in 0..n {
        if pe[i]
            .iter()
            .any(|&(pij, eij)| eij == e_max[p_index[&pij]] && max_count[p_index[&pij]] == 1)
        {
            result += 1;
        }
    }
    if result < n {
        result += 1;
    }
    println!("{}", result);
}
