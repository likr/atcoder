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
        xy: [(isize, isize); n],
    }
    let zw = xy
        .iter()
        .map(|&(xi, yi)| (xi + yi, xi - yi))
        .collect::<Vec<(isize, isize)>>();
    let z_min = zw.iter().map(|&(zi, _)| zi).min().unwrap();
    let z_max = zw.iter().map(|&(zi, _)| zi).max().unwrap();
    let w_min = zw.iter().map(|&(_, wi)| wi).min().unwrap();
    let w_max = zw.iter().map(|&(_, wi)| wi).max().unwrap();
    let d = max(z_max - z_min, w_max - w_min);
    debug!(d);
    let p = vec![
        (z_max - d / 2, w_max - d / 2),
        (z_max - d / 2, d / 2 + w_min),
        (d / 2 + z_min, w_max - d / 2),
        (d / 2 + z_min, d / 2 + w_min),
    ];
    for &(pz, pw) in &p {
        let px = (pz + pw) / 2;
        let py = (pz - pw) / 2;
        debug!(px, py, pz, pw);
        if xy
            .iter()
            .all(|(xi, yi)| (xi - px).abs() + (yi - py).abs() == d / 2)
        {
            println!("{} {}", px, py);
            return;
        }
    }
}
