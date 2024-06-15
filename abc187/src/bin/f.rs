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
    let mut edges = HashSet::new();
    for &(ai, bi) in ab.iter() {
        edges.insert((min(ai, bi), max(ai, bi)));
    }
    let mut clique = vec![true; 1 << n];
    for x in 0..1 << n {
        for j in 1..n {
            for i in 0..j {
                if x & 1 << i > 0 && x & 1 << j > 0 {
                    if !edges.contains(&(i, j)) {
                        clique[x] = false;
                    }
                }
            }
        }
    }
    let mut maximal = vec![];
    for x in 0..1 << n {
        if clique[x] {
            if (0..n)
                .filter(|&i| x & 1 << i == 0)
                .all(|i| !clique[x | 1 << i])
            {
                maximal.push(x);
            }
        }
    }
    let mut distance = vec![INF; 1 << n];
    distance[0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(x) = queue.pop_front() {
        for &y in maximal.iter() {
            if distance[x | y] == INF {
                distance[x | y] = distance[x] + 1;
                queue.push_back(x | y);
            }
        }
    }
    println!("{}", distance[(1 << n) - 1]);
}
