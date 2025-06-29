use permutohedron::LexicalPermutation;
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
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut nodes = (0..n).collect::<Vec<_>>();
    let mut edges = HashSet::new();
    for &(ai, bi) in ab.iter() {
        edges.insert((ai, bi));
        edges.insert((bi, ai));
    }
    let mut ans = 0;
    loop {
        if nodes[0] == 0 && (1..n).all(|i| edges.contains(&(nodes[i - 1], nodes[i]))) {
            ans += 1;
        }
        if !nodes.next_permutation() {
            break;
        }
    }
    println!("{}", ans);
}
