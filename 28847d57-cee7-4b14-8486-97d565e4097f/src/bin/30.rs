use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        m: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); m],
    }
    let mut graph1 = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph1[ai].push(bi);
        graph1[bi].push(ai);
    }
    for i in 0..n {
        graph1[i].sort();
    }

    let mut indices = (0..n).collect::<Vec<_>>();
    loop {
        let mut graph2 = vec![vec![]; n];
        for &(ci, di) in cd.iter() {
            graph2[indices[ci]].push(indices[di]);
            graph2[indices[di]].push(indices[ci]);
        }
        for i in 0..n {
            graph2[i].sort();
        }
        if graph1 == graph2 {
            println!("Yes");
            return;
        }
        if !indices.next_permutation() {
            break;
        }
    }
    println!("No");
}
