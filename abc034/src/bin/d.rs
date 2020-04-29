use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use std::f64::NEG_INFINITY;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut wp: [(usize, usize); n],
    }
    wp.sort_by_key(|&(_, pi)| pi);
    wp.reverse();
    let ws = wp
        .into_iter()
        .map(|(wi, pi)| (wi as f64, (wi * pi) as f64 / 100.))
        .collect::<Vec<_>>();
    let mut dp = vec![(NEG_INFINITY, 0., 0.); k + 1];
    dp[0] = (0., 0., 0.);
    for i in 0..n {
        let (wi, si) = ws[i];
        for j in (1..=k).rev() {
            if dp[j - 1].0 == NEG_INFINITY {
                continue;
            }
            let w = dp[j - 1].1 + wi;
            let s = dp[j - 1].2 + si;
            if s / w > dp[j].0 {
                dp[j] = (s / w, w, s);
            }
        }
    }
    println!("{}", dp[k].0 * 100.);
}
