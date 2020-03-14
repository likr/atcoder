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
        n: usize,
        m: usize,
        ab: [(isize, isize); n],
        cd: [(isize, isize); m],
    }
    for &(ai, bi) in &ab {
        let k = (0..m)
            .min_by_key(|&j| {
                let (cj, dj) = cd[j];
                (ai - cj).abs() + (bi - dj).abs()
            })
            .unwrap();
        println!("{}", k + 1);
    }
}
