use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        s: [String; n],
    }
    let mut count = HashMap::new();
    for si in &s {
        if !count.contains_key(&si) {
            count.insert(si, 0);
        }
        *count.get_mut(&si).unwrap() += 1;
    }
    let mut count = count.into_iter().map(|(k, v)| (v, k)).collect::<Vec<_>>();
    count.sort();
    count.reverse();
    let c = count[0].0;
    let count = count
        .into_iter()
        .filter(|(ci, _)| *ci == c)
        .collect::<Vec<_>>();
    for (_, si) in count.iter().rev() {
        println!("{}", si);
    }
}
