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
        a: [[Usize1; n - 1]; n],
    }
    let mut queues = a
        .iter()
        .map(|ai| ai.iter().map(|&aij| aij).collect::<VecDeque<usize>>())
        .collect::<Vec<_>>();
    let mut finished = HashSet::new();
    let mut candidates = (0..n).collect::<HashSet<_>>();
    let mut next_candidates = HashSet::new();
    for d in 1.. {
        next_candidates.clear();
        for &i in candidates.iter() {
            if next_candidates.contains(&i) || finished.contains(&i) {
                continue;
            }
            let j = *queues[i].front().unwrap();
            if next_candidates.contains(&j) || finished.contains(&j) {
                continue;
            }
            let k = *queues[j].front().unwrap();
            if i == k {
                queues[i].pop_front();
                if queues[i].is_empty() {
                    finished.insert(i);
                } else {
                    next_candidates.insert(i);
                }
                queues[j].pop_front();
                if queues[j].is_empty() {
                    finished.insert(j);
                } else {
                    next_candidates.insert(j);
                }
            }
        }
        if finished.len() == n {
            println!("{}", d);
            return;
        }
        if next_candidates.is_empty() {
            println!("-1");
            return;
        }
        debug!(next_candidates);
        debug!(queues);
        std::mem::swap(&mut candidates, &mut next_candidates);
    }
}
