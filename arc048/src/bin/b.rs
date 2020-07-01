use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        rh: [(usize, Usize1); n],
    }
    let mut r = rh.iter().map(|&(ri, _)| ri).collect::<Vec<_>>();
    r.sort();

    let mut hands = HashMap::new();
    for &(ri, hi) in &rh {
        hands.entry(ri).or_insert([0; 3])[hi] += 1;
    }

    for &(ri, hi) in &rh {
        let win = r.lower_bound(&ri);
        let lose = n - r.upper_bound(&ri);
        let hw = hands[&ri][(hi + 1) % 3];
        let hl = hands[&ri][(hi + 2) % 3];
        let hd = hands[&ri][hi] - 1;
        println!("{} {} {}", win + hw, lose + hl, hd);
    }
}
