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
        t:  usize,
        ab: [(Usize1, usize); t],
    }
    let mut count = HashMap::new();
    count.insert(0, n);
    let mut score = vec![0; n];
    for i in 0..t {
        let (ai, bi) = ab[i];
        *count.entry(score[ai]).or_insert(0) -= 1;
        if count[&score[ai]] == 0 {
            count.remove(&score[ai]);
        }
        score[ai] += bi;
        *count.entry(score[ai]).or_insert(0) += 1;
        println!("{}", count.len());
    }
}
