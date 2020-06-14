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

fn main() {
    input! {
        x: isize,
        n: usize,
        p: [isize; n],
    }
    let p = p.into_iter().collect::<HashSet<_>>();
    println!(
        "{}",
        (0..=101)
            .filter(|y| !p.contains(&y))
            .min_by_key(|&y| ((x - y).abs(), y))
            .unwrap()
    );
}
