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
        xc: [(i64, usize); n],
    }
    let mut cs = xc.iter().map(|&(_, ci)| ci).collect::<Vec<_>>();
    cs.sort();
    cs.dedup();
    let mut xs = cs.iter().map(|&c| (c, vec![])).collect::<HashMap<_, _>>();
    for i in 0..n {
        let (xi, ci) = xc[i];
        xs.get_mut(&ci).unwrap().push(xi);
    }
    let mut x_min = HashMap::new();
    let mut x_max = HashMap::new();
    for &c in cs.iter() {
        xs.get_mut(&c).unwrap().sort();
        x_min.insert(c, xs[&c][0]);
        x_max.insert(c, xs[&c][xs[&c].len() - 1]);
    }
    cs.reverse();
    cs.push(0);
    cs.reverse();
    x_min.insert(0, 0);
    x_max.insert(0, 0);
    let mut dp = vec![(0, 0)];
    for i in 1..cs.len() {
        let left0 = x_min[&cs[i - 1]];
        let right0 = x_max[&cs[i - 1]];
        let left_cost0 = dp[i - 1].0;
        let right_cost0 = dp[i - 1].1;
        let left1 = x_min[&cs[i]];
        let right1 = x_max[&cs[i]];
        let d = right1 - left1;
        let left_cost1 = min(
            left_cost0 + (right1 - left0).abs() + d,
            right_cost0 + (right1 - right0).abs() + d,
        );
        let right_cost1 = min(
            left_cost0 + (left1 - left0).abs() + d,
            right_cost0 + (left1 - right0).abs() + d,
        );
        dp.push((left_cost1, right_cost1));
    }
    println!(
        "{}",
        min(
            dp[dp.len() - 1].0 + x_min[&cs[cs.len() - 1]].abs(),
            dp[dp.len() - 1].1 + x_max[&cs[cs.len() - 1]].abs(),
        )
    );
}
