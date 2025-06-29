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
        k: usize,
        p: usize,
        ca: [(usize, [usize; k]); n],
    }
    let mut dp1 = HashMap::new();
    dp1.insert(vec![0; k], 0);
    for i in 0..n {
        let mut dp2 = dp1.clone();
        let (ci, ai) = &ca[i];
        for (key, &cost) in dp1.iter() {
            let new_key = (0..k).map(|j| min(p, key[j] + ai[j])).collect::<Vec<_>>();
            let v = dp2.entry(new_key).or_insert(INF);
            *v = min(cost + ci, *v);
        }
        std::mem::swap(&mut dp1, &mut dp2);
        // debug!(dp1);
    }
    if let Some(&cost) = dp1.get(&vec![p; k]) {
        println!("{}", cost);
    } else {
        println!("-1");
    }
}
