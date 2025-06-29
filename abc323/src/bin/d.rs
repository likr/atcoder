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
        sc: [(usize, usize); n],
    }
    let mut queue = BinaryHeap::new();
    let mut used = HashSet::new();
    let mut count = HashMap::new();
    for &(si, ci) in sc.iter() {
        queue.push(Reverse(si));
        used.insert(si);
        count.insert(si, ci);
    }
    let mut ans = 0usize;
    while let Some(Reverse(si)) = queue.pop() {
        if count[&si] % 2 == 1 {
            ans += 1;
        }
        if count[&si] / 2 > 0 {
            *count.entry(2 * si).or_insert(0) += count[&si] / 2;
            if count[&(si * 2)] > 0 && !used.contains(&(2 * si)) {
                queue.push(Reverse(2 * si));
                used.insert(2 * si);
            }
        }
        count.remove(&si).unwrap();
    }
    println!("{}", ans);
}
